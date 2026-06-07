#include "uds_protocol.hpp"

#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>
#include <cstring>
#include <chrono>
#include <thread>
#include <atomic>

#include "rclcpp/rclcpp.hpp"
#include "geometry_msgs/msg/twist.hpp"
#include "std_msgs/msg/float32_multi_array.hpp"
#include "std_msgs/msg/int32_multi_array.hpp"

using namespace std::chrono_literals;

class NavRos2Bridge : public rclcpp::Node
{
public:
    NavRos2Bridge()
        : Node("nav_ros2_bridge")
    {
        RCLCPP_INFO(get_logger(), "NavRos2Bridge starting...");

        // ====== 下行：底盘控制 ======
        uds_down_fd_ = socket(AF_UNIX, SOCK_DGRAM, 0);
        if (uds_down_fd_ < 0) {
            RCLCPP_ERROR(get_logger(), "Failed to create UDS down socket");
            return;
        }

        memset(&down_addr_, 0, sizeof(down_addr_));
        down_addr_.sun_family = AF_UNIX;
        strncpy(down_addr_.sun_path, UDS_SOCKET_PATH, sizeof(down_addr_.sun_path) - 1);

        cmd_vel_sub_ = create_subscription<geometry_msgs::msg::Twist>(
            "cmd_vel", 10,
            std::bind(&NavRos2Bridge::onCmdVel, this, std::placeholders::_1));

        // ====== 上行：接收 sk2026 自瞄上发的数据 ======
        uds_up_fd_ = socket(AF_UNIX, SOCK_DGRAM, 0);
        if (uds_up_fd_ < 0) {
            RCLCPP_ERROR(get_logger(), "Failed to create UDS up socket");
            return;
        }

        struct sockaddr_un up_addr;
        memset(&up_addr, 0, sizeof(up_addr));
        up_addr.sun_family = AF_UNIX;
        strncpy(up_addr.sun_path, UDS_SOCKET_UP_PATH, sizeof(up_addr.sun_path) - 1);

        if (connect(uds_up_fd_, (struct sockaddr*)&up_addr, sizeof(up_addr)) < 0) {
            RCLCPP_WARN(get_logger(), "UDS up connect failed, will retry: %s", strerror(errno));
        }

        // 上行数据发布器
        game_status_pub_ = create_publisher<std_msgs::msg::Int32MultiArray>("nav/game_status", 10);
        robot_hp_pub_ = create_publisher<std_msgs::msg::Int32MultiArray>("nav/robot_hp", 10);
        rfid_pub_ = create_publisher<std_msgs::msg::Int32MultiArray>("nav/rfid", 10);
        robot_status_pub_ = create_publisher<std_msgs::msg::Float32MultiArray>("nav/robot_status", 10);
        imu_pub_ = create_publisher<std_msgs::msg::Float32MultiArray>("nav/imu", 10);

        // 启动上行数据接收线程
        running_ = true;
        uds_up_thread_ = std::thread(&NavRos2Bridge::udsUpRecvLoop, this);

        RCLCPP_INFO(get_logger(), "NavRos2Bridge ready: chassis downlink + uplink receiver");
    }

    ~NavRos2Bridge()
    {
        running_ = false;
        if (uds_up_thread_.joinable()) {
            uds_up_thread_.join();
        }
        if (uds_down_fd_ >= 0) close(uds_down_fd_);
        if (uds_up_fd_ >= 0) close(uds_up_fd_);
    }

private:
    // ====== 下行 ======
    void sendDown(const UdsNavMessage& msg)
    {
        if (uds_down_fd_ < 0) return;
        sendto(uds_down_fd_, &msg, sizeof(msg), 0,
               (struct sockaddr*)&down_addr_, sizeof(down_addr_));
    }

    void onCmdVel(const geometry_msgs::msg::Twist::SharedPtr msg)
    {
        UdsNavMessage uds_msg;
        uds_msg.type = UDS_MSG_CHASSIS;
        uds_msg.data.chassis.vx = static_cast<float>(msg->linear.x);
        uds_msg.data.chassis.vy = static_cast<float>(msg->linear.y);
        uds_msg.data.chassis.wz = static_cast<float>(msg->angular.z);
        sendDown(uds_msg);
    }

    // ====== 上行 ======
    void udsUpRecvLoop()
    {
        UdsUpMessage up_msg;
        while (running_ && rclcpp::ok()) {
            // 重连机制
            if (uds_up_fd_ < 0) {
                uds_up_fd_ = socket(AF_UNIX, SOCK_DGRAM, 0);
                if (uds_up_fd_ < 0) {
                    std::this_thread::sleep_for(std::chrono::seconds(1));
                    continue;
                }
                struct sockaddr_un up_addr;
                memset(&up_addr, 0, sizeof(up_addr));
                up_addr.sun_family = AF_UNIX;
                strncpy(up_addr.sun_path, UDS_SOCKET_UP_PATH, sizeof(up_addr.sun_path) - 1);
                connect(uds_up_fd_, (struct sockaddr*)&up_addr, sizeof(up_addr));
            }

            ssize_t n = recv(uds_up_fd_, &up_msg, sizeof(up_msg), 0);
            if (n != sizeof(up_msg)) {
                if (n < 0 && errno != EAGAIN && errno != EWOULDBLOCK) {
                    RCLCPP_WARN_THROTTLE(get_logger(), *get_clock(), 5000,
                                         "UDS up recv error: %s", strerror(errno));
                }
                std::this_thread::sleep_for(std::chrono::milliseconds(10));
                continue;
            }

            switch (up_msg.type) {
                case UDS_UP_GAME_STATUS: {
                    auto msg = std_msgs::msg::Int32MultiArray();
                    msg.data = {
                        static_cast<int32_t>(up_msg.data.game_status.game_progress),
                        static_cast<int32_t>(up_msg.data.game_status.stage_remain_time),
                        static_cast<int32_t>(up_msg.data.game_status.time_stamp)
                    };
                    game_status_pub_->publish(msg);
                    break;
                }
                case UDS_UP_ALL_ROBOT_HP: {
                    auto msg = std_msgs::msg::Int32MultiArray();
                    msg.data = {
                        static_cast<int32_t>(up_msg.data.all_robot_hp.robot_id),
                        up_msg.data.all_robot_hp.blue_1_robot_HP,
                        up_msg.data.all_robot_hp.blue_2_robot_HP,
                        up_msg.data.all_robot_hp.blue_3_robot_HP,
                        up_msg.data.all_robot_hp.blue_4_robot_HP,
                        up_msg.data.all_robot_hp.blue_5_robot_HP,
                        up_msg.data.all_robot_hp.blue_7_robot_HP,
                        up_msg.data.all_robot_hp.red_1_robot_HP,
                        up_msg.data.all_robot_hp.red_2_robot_HP,
                        up_msg.data.all_robot_hp.red_3_robot_HP,
                        up_msg.data.all_robot_hp.red_4_robot_HP,
                        up_msg.data.all_robot_hp.red_5_robot_HP,
                        up_msg.data.all_robot_hp.red_7_robot_HP,
                        up_msg.data.all_robot_hp.blue_outpost_HP,
                        up_msg.data.all_robot_hp.red_outpost_HP,
                        up_msg.data.all_robot_hp.blue_base_HP,
                        up_msg.data.all_robot_hp.red_base_HP
                    };
                    robot_hp_pub_->publish(msg);
                    break;
                }
                case UDS_UP_RFID_STATUS: {
                    auto msg = std_msgs::msg::Int32MultiArray();
                    msg.data = {
                        static_cast<int32_t>(up_msg.data.rfid_status.time_stamp),
                        static_cast<int32_t>(up_msg.data.rfid_status.rfid_bits)
                    };
                    rfid_pub_->publish(msg);
                    break;
                }
                case UDS_UP_ROBOT_STATUS_DETAIL: {
                    auto msg = std_msgs::msg::Float32MultiArray();
                    msg.data = {
                        static_cast<float>(up_msg.data.robot_status_detail.time_stamp),
                        static_cast<float>(up_msg.data.robot_status_detail.robot_id),
                        static_cast<float>(up_msg.data.robot_status_detail.robot_level),
                        static_cast<float>(up_msg.data.robot_status_detail.current_hp),
                        static_cast<float>(up_msg.data.robot_status_detail.maximum_hp),
                        static_cast<float>(up_msg.data.robot_status_detail.shooter_barrel_cooling_value),
                        static_cast<float>(up_msg.data.robot_status_detail.shooter_barrel_heat_limit),
                        static_cast<float>(up_msg.data.robot_status_detail.shooter_17mm_1_barrel_heat),
                        up_msg.data.robot_status_detail.robot_pos_x,
                        up_msg.data.robot_status_detail.robot_pos_y,
                        up_msg.data.robot_status_detail.robot_pos_angle,
                        static_cast<float>(up_msg.data.robot_status_detail.armor_id),
                        static_cast<float>(up_msg.data.robot_status_detail.hp_deduction_reason),
                        static_cast<float>(up_msg.data.robot_status_detail.projectile_allowance_17mm),
                        static_cast<float>(up_msg.data.robot_status_detail.remaining_gold_coin)
                    };
                    robot_status_pub_->publish(msg);
                    break;
                }
                case UDS_UP_IMU: {
                    auto msg = std_msgs::msg::Float32MultiArray();
                    msg.data = {
                        up_msg.data.imu.yaw,
                        up_msg.data.imu.pitch,
                        up_msg.data.imu.roll,
                        up_msg.data.imu.yaw_vel,
                        up_msg.data.imu.pitch_vel,
                        up_msg.data.imu.roll_vel
                    };
                    imu_pub_->publish(msg);
                    break;
                }
                default:
                    break;
            }
        }
        RCLCPP_INFO(get_logger(), "UDS uplink receiver thread exiting");
    }

    // ====== 下行 ======
    int uds_down_fd_ = -1;
    struct sockaddr_un down_addr_;
    rclcpp::Subscription<geometry_msgs::msg::Twist>::SharedPtr cmd_vel_sub_;

    // ====== 上行 ======
    int uds_up_fd_ = -1;
    std::atomic<bool> running_{false};
    std::thread uds_up_thread_;

    rclcpp::Publisher<std_msgs::msg::Int32MultiArray>::SharedPtr game_status_pub_;
    rclcpp::Publisher<std_msgs::msg::Int32MultiArray>::SharedPtr robot_hp_pub_;
    rclcpp::Publisher<std_msgs::msg::Int32MultiArray>::SharedPtr rfid_pub_;
    rclcpp::Publisher<std_msgs::msg::Float32MultiArray>::SharedPtr robot_status_pub_;
    rclcpp::Publisher<std_msgs::msg::Float32MultiArray>::SharedPtr imu_pub_;
};

int main(int argc, char* argv[])
{
    rclcpp::init(argc, argv);
    auto node = std::make_shared<NavRos2Bridge>();
    rclcpp::spin(node);
    rclcpp::shutdown();
    return 0;
}