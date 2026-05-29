#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "auto_aim_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__auto_aim_interfaces__msg__Armor() -> *const std::ffi::c_void;
}

#[link(name = "auto_aim_interfaces__rosidl_generator_c")]
extern "C" {
    fn auto_aim_interfaces__msg__Armor__init(msg: *mut Armor) -> bool;
    fn auto_aim_interfaces__msg__Armor__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Armor>, size: usize) -> bool;
    fn auto_aim_interfaces__msg__Armor__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Armor>);
    fn auto_aim_interfaces__msg__Armor__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Armor>, out_seq: *mut rosidl_runtime_rs::Sequence<Armor>) -> bool;
}

// Corresponds to auto_aim_interfaces__msg__Armor
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Armor {

    // This member is not documented.
    #[allow(missing_docs)]
    pub number: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance_to_image_center: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::Pose,

}



impl Default for Armor {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !auto_aim_interfaces__msg__Armor__init(&mut msg as *mut _) {
        panic!("Call to auto_aim_interfaces__msg__Armor__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Armor {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__Armor__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__Armor__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__Armor__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Armor {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Armor where Self: Sized {
  const TYPE_NAME: &'static str = "auto_aim_interfaces/msg/Armor";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__auto_aim_interfaces__msg__Armor() }
  }
}


#[link(name = "auto_aim_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__auto_aim_interfaces__msg__Armors() -> *const std::ffi::c_void;
}

#[link(name = "auto_aim_interfaces__rosidl_generator_c")]
extern "C" {
    fn auto_aim_interfaces__msg__Armors__init(msg: *mut Armors) -> bool;
    fn auto_aim_interfaces__msg__Armors__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Armors>, size: usize) -> bool;
    fn auto_aim_interfaces__msg__Armors__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Armors>);
    fn auto_aim_interfaces__msg__Armors__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Armors>, out_seq: *mut rosidl_runtime_rs::Sequence<Armors>) -> bool;
}

// Corresponds to auto_aim_interfaces__msg__Armors
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Armors {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub armors: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Armor>,

}



impl Default for Armors {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !auto_aim_interfaces__msg__Armors__init(&mut msg as *mut _) {
        panic!("Call to auto_aim_interfaces__msg__Armors__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Armors {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__Armors__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__Armors__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__Armors__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Armors {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Armors where Self: Sized {
  const TYPE_NAME: &'static str = "auto_aim_interfaces/msg/Armors";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__auto_aim_interfaces__msg__Armors() }
  }
}


#[link(name = "auto_aim_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__auto_aim_interfaces__msg__Target() -> *const std::ffi::c_void;
}

#[link(name = "auto_aim_interfaces__rosidl_generator_c")]
extern "C" {
    fn auto_aim_interfaces__msg__Target__init(msg: *mut Target) -> bool;
    fn auto_aim_interfaces__msg__Target__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Target>, size: usize) -> bool;
    fn auto_aim_interfaces__msg__Target__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Target>);
    fn auto_aim_interfaces__msg__Target__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Target>, out_seq: *mut rosidl_runtime_rs::Sequence<Target>) -> bool;
}

// Corresponds to auto_aim_interfaces__msg__Target
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Target {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tracking: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub id: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub armors_num: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position: geometry_msgs::msg::rmw::Point,


    // This member is not documented.
    #[allow(missing_docs)]
    pub velocity: geometry_msgs::msg::rmw::Vector3,


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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !auto_aim_interfaces__msg__Target__init(&mut msg as *mut _) {
        panic!("Call to auto_aim_interfaces__msg__Target__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Target {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__Target__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__Target__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__Target__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Target {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Target where Self: Sized {
  const TYPE_NAME: &'static str = "auto_aim_interfaces/msg/Target";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__auto_aim_interfaces__msg__Target() }
  }
}


#[link(name = "auto_aim_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__auto_aim_interfaces__msg__DebugLight() -> *const std::ffi::c_void;
}

#[link(name = "auto_aim_interfaces__rosidl_generator_c")]
extern "C" {
    fn auto_aim_interfaces__msg__DebugLight__init(msg: *mut DebugLight) -> bool;
    fn auto_aim_interfaces__msg__DebugLight__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DebugLight>, size: usize) -> bool;
    fn auto_aim_interfaces__msg__DebugLight__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DebugLight>);
    fn auto_aim_interfaces__msg__DebugLight__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DebugLight>, out_seq: *mut rosidl_runtime_rs::Sequence<DebugLight>) -> bool;
}

// Corresponds to auto_aim_interfaces__msg__DebugLight
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !auto_aim_interfaces__msg__DebugLight__init(&mut msg as *mut _) {
        panic!("Call to auto_aim_interfaces__msg__DebugLight__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DebugLight {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__DebugLight__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__DebugLight__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__DebugLight__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DebugLight {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DebugLight where Self: Sized {
  const TYPE_NAME: &'static str = "auto_aim_interfaces/msg/DebugLight";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__auto_aim_interfaces__msg__DebugLight() }
  }
}


#[link(name = "auto_aim_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__auto_aim_interfaces__msg__DebugLights() -> *const std::ffi::c_void;
}

#[link(name = "auto_aim_interfaces__rosidl_generator_c")]
extern "C" {
    fn auto_aim_interfaces__msg__DebugLights__init(msg: *mut DebugLights) -> bool;
    fn auto_aim_interfaces__msg__DebugLights__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DebugLights>, size: usize) -> bool;
    fn auto_aim_interfaces__msg__DebugLights__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DebugLights>);
    fn auto_aim_interfaces__msg__DebugLights__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DebugLights>, out_seq: *mut rosidl_runtime_rs::Sequence<DebugLights>) -> bool;
}

// Corresponds to auto_aim_interfaces__msg__DebugLights
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DebugLights {

    // This member is not documented.
    #[allow(missing_docs)]
    pub data: rosidl_runtime_rs::Sequence<super::super::msg::rmw::DebugLight>,

}



impl Default for DebugLights {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !auto_aim_interfaces__msg__DebugLights__init(&mut msg as *mut _) {
        panic!("Call to auto_aim_interfaces__msg__DebugLights__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DebugLights {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__DebugLights__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__DebugLights__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__DebugLights__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DebugLights {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DebugLights where Self: Sized {
  const TYPE_NAME: &'static str = "auto_aim_interfaces/msg/DebugLights";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__auto_aim_interfaces__msg__DebugLights() }
  }
}


#[link(name = "auto_aim_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__auto_aim_interfaces__msg__DebugArmor() -> *const std::ffi::c_void;
}

#[link(name = "auto_aim_interfaces__rosidl_generator_c")]
extern "C" {
    fn auto_aim_interfaces__msg__DebugArmor__init(msg: *mut DebugArmor) -> bool;
    fn auto_aim_interfaces__msg__DebugArmor__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DebugArmor>, size: usize) -> bool;
    fn auto_aim_interfaces__msg__DebugArmor__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DebugArmor>);
    fn auto_aim_interfaces__msg__DebugArmor__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DebugArmor>, out_seq: *mut rosidl_runtime_rs::Sequence<DebugArmor>) -> bool;
}

// Corresponds to auto_aim_interfaces__msg__DebugArmor
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DebugArmor {

    // This member is not documented.
    #[allow(missing_docs)]
    pub center_x: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: rosidl_runtime_rs::String,


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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !auto_aim_interfaces__msg__DebugArmor__init(&mut msg as *mut _) {
        panic!("Call to auto_aim_interfaces__msg__DebugArmor__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DebugArmor {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__DebugArmor__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__DebugArmor__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__DebugArmor__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DebugArmor {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DebugArmor where Self: Sized {
  const TYPE_NAME: &'static str = "auto_aim_interfaces/msg/DebugArmor";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__auto_aim_interfaces__msg__DebugArmor() }
  }
}


#[link(name = "auto_aim_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__auto_aim_interfaces__msg__DebugArmors() -> *const std::ffi::c_void;
}

#[link(name = "auto_aim_interfaces__rosidl_generator_c")]
extern "C" {
    fn auto_aim_interfaces__msg__DebugArmors__init(msg: *mut DebugArmors) -> bool;
    fn auto_aim_interfaces__msg__DebugArmors__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DebugArmors>, size: usize) -> bool;
    fn auto_aim_interfaces__msg__DebugArmors__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DebugArmors>);
    fn auto_aim_interfaces__msg__DebugArmors__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DebugArmors>, out_seq: *mut rosidl_runtime_rs::Sequence<DebugArmors>) -> bool;
}

// Corresponds to auto_aim_interfaces__msg__DebugArmors
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DebugArmors {

    // This member is not documented.
    #[allow(missing_docs)]
    pub data: rosidl_runtime_rs::Sequence<super::super::msg::rmw::DebugArmor>,

}



impl Default for DebugArmors {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !auto_aim_interfaces__msg__DebugArmors__init(&mut msg as *mut _) {
        panic!("Call to auto_aim_interfaces__msg__DebugArmors__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DebugArmors {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__DebugArmors__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__DebugArmors__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__DebugArmors__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DebugArmors {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DebugArmors where Self: Sized {
  const TYPE_NAME: &'static str = "auto_aim_interfaces/msg/DebugArmors";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__auto_aim_interfaces__msg__DebugArmors() }
  }
}


#[link(name = "auto_aim_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__auto_aim_interfaces__msg__TrackerInfo() -> *const std::ffi::c_void;
}

#[link(name = "auto_aim_interfaces__rosidl_generator_c")]
extern "C" {
    fn auto_aim_interfaces__msg__TrackerInfo__init(msg: *mut TrackerInfo) -> bool;
    fn auto_aim_interfaces__msg__TrackerInfo__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TrackerInfo>, size: usize) -> bool;
    fn auto_aim_interfaces__msg__TrackerInfo__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TrackerInfo>);
    fn auto_aim_interfaces__msg__TrackerInfo__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TrackerInfo>, out_seq: *mut rosidl_runtime_rs::Sequence<TrackerInfo>) -> bool;
}

// Corresponds to auto_aim_interfaces__msg__TrackerInfo
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Difference between the current measurement and prediction

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrackerInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub position_diff: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_diff: f64,

    /// Unfiltered position and yaw
    pub position: geometry_msgs::msg::rmw::Point,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw: f64,

}



impl Default for TrackerInfo {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !auto_aim_interfaces__msg__TrackerInfo__init(&mut msg as *mut _) {
        panic!("Call to auto_aim_interfaces__msg__TrackerInfo__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TrackerInfo {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__TrackerInfo__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__TrackerInfo__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { auto_aim_interfaces__msg__TrackerInfo__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TrackerInfo {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TrackerInfo where Self: Sized {
  const TYPE_NAME: &'static str = "auto_aim_interfaces/msg/TrackerInfo";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__auto_aim_interfaces__msg__TrackerInfo() }
  }
}


