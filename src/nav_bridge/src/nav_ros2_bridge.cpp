#include "uds_protocol.hpp"

#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>
#include <cstring>
#include <chrono>

#include "rclcpp/rclcpp.hpp"
#include "geometry_msgs/msg/twist.hpp"
#include "sensor_msgs/msg/joint_state.hpp"
#include "example_interfaces/msg/u_int8.hpp"
#include "auto_aim_interfaces/msg/target.hpp"

using namespace std::chrono_literals;

class NavRos2Bridge : public rclcpp::Node
{
public:
    NavRos2Bridge()
        : Node("nav_ros2_bridge")
    {
        RCLCPP_INFO(get_logger(), "NavRos2Bridge starting...");

        uds_fd_ = socket(AF_UNIX, SOCK_DGRAM, 0);
        if (uds_fd_ < 0) {
            RCLCPP_ERROR(get_logger(), "Failed to create UDS socket");
            return;
        }

        memset(&server_addr_, 0, sizeof(server_addr_));
        server_addr_.sun_family = AF_UNIX;
        strncpy(server_addr_.sun_path, UDS_SOCKET_PATH, sizeof(server_addr_.sun_path) - 1);

        cmd_vel_sub_ = create_subscription<geometry_msgs::msg::Twist>(
            "cmd_vel", 10,
            std::bind(&NavRos2Bridge::onCmdVel, this, std::placeholders::_1));

        cmd_gimbal_joint_sub_ = create_subscription<sensor_msgs::msg::JointState>(
            "cmd_gimbal_joint", 10,
            std::bind(&NavRos2Bridge::onCmdGimbalJoint, this, std::placeholders::_1));

        cmd_shoot_sub_ = create_subscription<example_interfaces::msg::UInt8>(
            "cmd_shoot", 10,
            std::bind(&NavRos2Bridge::onCmdShoot, this, std::placeholders::_1));

        cmd_tracking_sub_ = create_subscription<auto_aim_interfaces::msg::Target>(
            "tracker/target", 10,
            std::bind(&NavRos2Bridge::onTrackerTarget, this, std::placeholders::_1));

        RCLCPP_INFO(get_logger(), "NavRos2Bridge ready, subscribed to 4 topics");
    }

    ~NavRos2Bridge()
    {
        if (uds_fd_ >= 0) {
            close(uds_fd_);
        }
    }

private:
    void sendToUds(const UdsNavMessage& msg)
    {
        if (uds_fd_ < 0) return;

        ssize_t n = sendto(uds_fd_, &msg, sizeof(msg), 0,
                           (struct sockaddr*)&server_addr_, sizeof(server_addr_));
        if (n < 0) {
            RCLCPP_WARN_THROTTLE(get_logger(), *get_clock(), 2000,
                                 "UDS sendto failed");
        }
    }

    void onCmdVel(const geometry_msgs::msg::Twist::SharedPtr msg)
    {
        UdsNavMessage uds_msg;
        uds_msg.type = UDS_MSG_CHASSIS;
        uds_msg.data.chassis.vx = static_cast<float>(msg->linear.x);
        uds_msg.data.chassis.vy = static_cast<float>(msg->linear.y);
        uds_msg.data.chassis.wz = static_cast<float>(msg->angular.z);
        sendToUds(uds_msg);
    }

    void onCmdGimbalJoint(const sensor_msgs::msg::JointState::SharedPtr msg)
    {
        if (msg->name.size() != msg->position.size()) {
            return;
        }

        UdsNavMessage uds_msg;
        uds_msg.type = UDS_MSG_GIMBAL;
        uds_msg.data.gimbal.pitch = 0.0f;
        uds_msg.data.gimbal.yaw = 0.0f;

        for (size_t i = 0; i < msg->name.size(); ++i) {
            if (msg->name[i] == "gimbal_pitch_joint") {
                uds_msg.data.gimbal.pitch = static_cast<float>(msg->position[i]);
            } else if (msg->name[i] == "gimbal_yaw_joint") {
                uds_msg.data.gimbal.yaw = static_cast<float>(msg->position[i]);
            }
        }
        sendToUds(uds_msg);
    }

    void onCmdShoot(const example_interfaces::msg::UInt8::SharedPtr msg)
    {
        UdsNavMessage uds_msg;
        uds_msg.type = UDS_MSG_SHOOT;
        uds_msg.data.shoot.fire = msg->data;
        uds_msg.data.shoot.fric_on = 1;
        sendToUds(uds_msg);
    }

    void onTrackerTarget(const auto_aim_interfaces::msg::Target::SharedPtr msg)
    {
        UdsNavMessage uds_msg;
        uds_msg.type = UDS_MSG_TRACKING;
        uds_msg.data.track.tracking = msg->tracking ? 1 : 0;
        sendToUds(uds_msg);
    }

    int uds_fd_ = -1;
    struct sockaddr_un server_addr_;

    rclcpp::Subscription<geometry_msgs::msg::Twist>::SharedPtr cmd_vel_sub_;
    rclcpp::Subscription<sensor_msgs::msg::JointState>::SharedPtr cmd_gimbal_joint_sub_;
    rclcpp::Subscription<example_interfaces::msg::UInt8>::SharedPtr cmd_shoot_sub_;
    rclcpp::Subscription<auto_aim_interfaces::msg::Target>::SharedPtr cmd_tracking_sub_;
};

int main(int argc, char* argv[])
{
    rclcpp::init(argc, argv);
    auto node = std::make_shared<NavRos2Bridge>();
    rclcpp::spin(node);
    rclcpp::shutdown();
    return 0;
}