#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to auto_aim_interfaces__msg__Armor

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Armor {

    // This member is not documented.
    #[allow(missing_docs)]
    pub number: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance_to_image_center: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::Pose,

}



impl Default for Armor {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Armor::default())
  }
}

impl rosidl_runtime_rs::Message for Armor {
  type RmwMsg = super::msg::rmw::Armor;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        number: msg.number.as_str().into(),
        type_: msg.type_.as_str().into(),
        distance_to_image_center: msg.distance_to_image_center,
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        number: msg.number.as_str().into(),
        type_: msg.type_.as_str().into(),
      distance_to_image_center: msg.distance_to_image_center,
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      number: msg.number.to_string(),
      type_: msg.type_.to_string(),
      distance_to_image_center: msg.distance_to_image_center,
      pose: geometry_msgs::msg::Pose::from_rmw_message(msg.pose),
    }
  }
}


// Corresponds to auto_aim_interfaces__msg__Armors

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Armors {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub armors: Vec<super::msg::Armor>,

}



impl Default for Armors {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Armors::default())
  }
}

impl rosidl_runtime_rs::Message for Armors {
  type RmwMsg = super::msg::rmw::Armors;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        armors: msg.armors
          .into_iter()
          .map(|elem| super::msg::Armor::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        armors: msg.armors
          .iter()
          .map(|elem| super::msg::Armor::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      armors: msg.armors
          .into_iter()
          .map(super::msg::Armor::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to auto_aim_interfaces__msg__Target

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Target {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tracking: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub id: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub armors_num: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position: geometry_msgs::msg::Point,


    // This member is not documented.
    #[allow(missing_docs)]
    pub velocity: geometry_msgs::msg::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub v_yaw: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub radius_1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub radius_2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dz: f64,

}



impl Default for Target {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Target::default())
  }
}

impl rosidl_runtime_rs::Message for Target {
  type RmwMsg = super::msg::rmw::Target;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        tracking: msg.tracking,
        id: msg.id.as_str().into(),
        armors_num: msg.armors_num,
        position: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.position)).into_owned(),
        velocity: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.velocity)).into_owned(),
        yaw: msg.yaw,
        v_yaw: msg.v_yaw,
        radius_1: msg.radius_1,
        radius_2: msg.radius_2,
        dz: msg.dz,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      tracking: msg.tracking,
        id: msg.id.as_str().into(),
      armors_num: msg.armors_num,
        position: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.position)).into_owned(),
        velocity: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.velocity)).into_owned(),
      yaw: msg.yaw,
      v_yaw: msg.v_yaw,
      radius_1: msg.radius_1,
      radius_2: msg.radius_2,
      dz: msg.dz,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      tracking: msg.tracking,
      id: msg.id.to_string(),
      armors_num: msg.armors_num,
      position: geometry_msgs::msg::Point::from_rmw_message(msg.position),
      velocity: geometry_msgs::msg::Vector3::from_rmw_message(msg.velocity),
      yaw: msg.yaw,
      v_yaw: msg.v_yaw,
      radius_1: msg.radius_1,
      radius_2: msg.radius_2,
      dz: msg.dz,
    }
  }
}


// Corresponds to auto_aim_interfaces__msg__DebugLight

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DebugLight {

    // This member is not documented.
    #[allow(missing_docs)]
    pub center_x: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_light: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ratio: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub angle: f32,

}



impl Default for DebugLight {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DebugLight::default())
  }
}

impl rosidl_runtime_rs::Message for DebugLight {
  type RmwMsg = super::msg::rmw::DebugLight;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        center_x: msg.center_x,
        is_light: msg.is_light,
        ratio: msg.ratio,
        angle: msg.angle,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      center_x: msg.center_x,
      is_light: msg.is_light,
      ratio: msg.ratio,
      angle: msg.angle,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      center_x: msg.center_x,
      is_light: msg.is_light,
      ratio: msg.ratio,
      angle: msg.angle,
    }
  }
}


// Corresponds to auto_aim_interfaces__msg__DebugLights

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DebugLights {

    // This member is not documented.
    #[allow(missing_docs)]
    pub data: Vec<super::msg::DebugLight>,

}



impl Default for DebugLights {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DebugLights::default())
  }
}

impl rosidl_runtime_rs::Message for DebugLights {
  type RmwMsg = super::msg::rmw::DebugLights;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        data: msg.data
          .into_iter()
          .map(|elem| super::msg::DebugLight::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        data: msg.data
          .iter()
          .map(|elem| super::msg::DebugLight::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      data: msg.data
          .into_iter()
          .map(super::msg::DebugLight::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to auto_aim_interfaces__msg__DebugArmor

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DebugArmor {

    // This member is not documented.
    #[allow(missing_docs)]
    pub center_x: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub light_ratio: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub center_distance: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub angle: f32,

}



impl Default for DebugArmor {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DebugArmor::default())
  }
}

impl rosidl_runtime_rs::Message for DebugArmor {
  type RmwMsg = super::msg::rmw::DebugArmor;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        center_x: msg.center_x,
        type_: msg.type_.as_str().into(),
        light_ratio: msg.light_ratio,
        center_distance: msg.center_distance,
        angle: msg.angle,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      center_x: msg.center_x,
        type_: msg.type_.as_str().into(),
      light_ratio: msg.light_ratio,
      center_distance: msg.center_distance,
      angle: msg.angle,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      center_x: msg.center_x,
      type_: msg.type_.to_string(),
      light_ratio: msg.light_ratio,
      center_distance: msg.center_distance,
      angle: msg.angle,
    }
  }
}


// Corresponds to auto_aim_interfaces__msg__DebugArmors

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DebugArmors {

    // This member is not documented.
    #[allow(missing_docs)]
    pub data: Vec<super::msg::DebugArmor>,

}



impl Default for DebugArmors {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DebugArmors::default())
  }
}

impl rosidl_runtime_rs::Message for DebugArmors {
  type RmwMsg = super::msg::rmw::DebugArmors;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        data: msg.data
          .into_iter()
          .map(|elem| super::msg::DebugArmor::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        data: msg.data
          .iter()
          .map(|elem| super::msg::DebugArmor::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      data: msg.data
          .into_iter()
          .map(super::msg::DebugArmor::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to auto_aim_interfaces__msg__TrackerInfo
/// Difference between the current measurement and prediction

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrackerInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub position_diff: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_diff: f64,

    /// Unfiltered position and yaw
    pub position: geometry_msgs::msg::Point,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw: f64,

}



impl Default for TrackerInfo {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TrackerInfo::default())
  }
}

impl rosidl_runtime_rs::Message for TrackerInfo {
  type RmwMsg = super::msg::rmw::TrackerInfo;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        position_diff: msg.position_diff,
        yaw_diff: msg.yaw_diff,
        position: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.position)).into_owned(),
        yaw: msg.yaw,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      position_diff: msg.position_diff,
      yaw_diff: msg.yaw_diff,
        position: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.position)).into_owned(),
      yaw: msg.yaw,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      position_diff: msg.position_diff,
      yaw_diff: msg.yaw_diff,
      position: geometry_msgs::msg::Point::from_rmw_message(msg.position),
      yaw: msg.yaw,
    }
  }
}


