use std::{clone::Clone, fmt, marker::PhantomData, str};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub type AssetId = u64;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum CreatorType {
    User,
    Group,
}
impl fmt::Display for CreatorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CreatorType::User => "User",
                CreatorType::Group => "Group",
            }
        )
    }
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Debug)]
#[repr(u8)]
pub enum AssetTypeId {
    Image = 1,
    TShirt = 2,
    Audio = 3,
    Mesh = 4,
    Lua = 5,
    Hat = 8,
    Place = 9,
    Model = 10,
    Shirt = 11,
    Pants = 12,
    Decal = 13,
    Head = 17,
    Face = 18,
    Gear = 19,
    Badge = 21,
    Animation = 24,
    Torso = 27,
    RightArm = 28,
    LeftArm = 29,
    LeftLeg = 30,
    RightLeg = 31,
    Package = 32,
    GamePass = 34,
    Plugin = 38,
    MeshPart = 40,
    HairAccessory = 41,
    FaceAccessory = 42,
    NeckAccessory = 43,
    ShoulderAccessory = 44,
    FrontAccessory = 45,
    BackAccessory = 46,
    WaistAccessory = 47,
    ClimbAnimation = 48,
    DeathAnimation = 49,
    FallAnimation = 50,
    IdleAnimation = 51,
    JumpAnimation = 52,
    RunAnimation = 53,
    SwimAnimation = 54,
    WalkAnimation = 55,
    PoseAnimation = 56,
    EarAccessory = 57,
    EyeAccessory = 58,
    EmoteAnimation = 61,
    Video = 62,
    TShirtAccessory = 64,
    ShirtAccessory = 65,
    PantsAccessory = 66,
    JacketAccessory = 67,
    SweaterAccessory = 68,
    ShortsAccessory = 69,
    LeftShoeAccessory = 70,
    RightShoeAccessory = 71,
    DressSkirtAccessory = 72,
}
impl fmt::Display for AssetTypeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap(),)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SocialSlotType {
    Automatic,
    Empty,
    Custom,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadImageResponse {
    pub target_id: AssetId,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Group;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Role;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Id<T> {
    id: AssetId,
    marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(primitive: AssetId) -> Self {
        Id {
            id: primitive,
            marker: PhantomData,
        }
    }

    pub fn into_inner(self) -> AssetId {
        self.id
    }
    
}
impl<T> From<AssetId> for Id<T> {
    fn from(id: AssetId) -> Self {
        Self::new(id)
    }
}
impl<T> From<Option<AssetId>> for Id<T> {
    fn from(id: Option<AssetId>) -> Self {
        Self::new(id.unwrap())
    }
}

//required for `std::option::Option<u64>` to implement `Into<std::option::Option<Id<rbx_api::models::Group>>>`

impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.id;
        fmt::Display::fmt(&inner, f)
    }
}
