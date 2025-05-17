pub mod sunshine_enum{
    pub enum SunshineType {
        CommonSunShine,
        // SFSUNSHINE, //sunflow's sunshine
    }
}


pub mod button_enum{
    #[derive(PartialEq)]
    pub enum ButtonType {
        GameStart,
        GamePause,
        GamePlaying,
        GameRestart,
        GameBack,
        None,
    }
    
    pub enum ButtonStatus {
        ButtonUp,
        ButtonDown,
    }
}

pub mod car_enum{
    pub enum CarStatus{
        Stopping,
        Running,
    }
}

pub mod card_enum{
    use super::plant_enum::PlantType;
    #[derive(PartialEq)]
    pub enum CardType{
        PeashooterCard,
        SunFlowerCard,
        WallnutCard,
        SpadeCard,
        NoneCard,
    }
    
    impl CardType {
        pub fn type_to_index(&self)->Option<usize>{
            match self {
                CardType::SpadeCard=>Some(0),
                CardType::PeashooterCard=>Some(1),
                CardType::SunFlowerCard=>Some(2),
                CardType::WallnutCard=>Some(3),
                CardType::NoneCard=>None,
            }
        }

        pub fn type_to_price(&self)->u32{
            match self{
                CardType::PeashooterCard=>100,
                CardType::SunFlowerCard=>50,
                CardType::WallnutCard=>50,
                _=>0,
            }
        }

        //通过”卡片类型“获取”植物“，如果不是”植物卡片“，返回None
        pub fn type_to_plant(&self)->Option<PlantType>{
            match self{
                CardType::PeashooterCard=>Some(PlantType::Peashooter),
                CardType::SunFlowerCard=>Some(PlantType::SunFlower),
                CardType::WallnutCard=>Some(PlantType::WallNut),
                _=>None,
            }
        }

        pub fn type_to_cool_time(&self)->f32{
            match self{
                CardType::PeashooterCard=>400.0,
                CardType::SunFlowerCard=>800.0,
                CardType::WallnutCard=>400.0,
                _=>0.0,
            }
        }
    }
}

pub mod plant_enum{
    #[derive(Clone)]
    pub enum PlantType{
        Peashooter,
        SunFlower,
        WallNut,
        NonePlant,
    }

    impl PlantType {
        pub fn type_to_index(&self)->Option<usize>{
            match self {
                PlantType::Peashooter=>Some(0),
                PlantType::SunFlower=>Some(1),
                PlantType::WallNut=>Some(2),
                PlantType::NonePlant=>None,
            }
        }

        pub fn type_to_frame_num(&self)->usize{
            match self {
                PlantType::Peashooter=>13,
                PlantType::SunFlower=>18,
                PlantType::WallNut=>15,
                PlantType::NonePlant=>0,
            }
        }

        pub fn type_to_blood(&self)->f32{
            match self {
                PlantType::Peashooter=>200.0,
                PlantType::SunFlower=>100.0,
                PlantType::WallNut=>400.0,
                PlantType::NonePlant=>0.0,
            }
        }

        pub fn type_to_skill_time(&self)->i32{
            match self{
                PlantType::Peashooter=>300,
                PlantType::SunFlower=>1500,
                _=>0,
            }
        }

        pub fn type_to_damage(&self)->f32{
            match self {
                PlantType::Peashooter=>50.0,
                _=>0.0,
            }
        }

    }
}



pub mod zombie_enum{
    pub enum ZombieType {
        CommonZM,
        ConeHeadZM,
        PoleVaultingZM,
    }

    impl ZombieType{
        pub fn type_to_index(&self)->usize{
            match self{
                Self::CommonZM=>0,
                Self::ConeHeadZM=>1,
                Self::PoleVaultingZM=>2,
            }
        }

        pub fn type_to_width_height(&self)->(f32,f32){
            match self {
                Self::CommonZM=>(200.0,200.0),
                Self::ConeHeadZM=>(200.0,200.0),
                Self::PoleVaultingZM=>(200.0,200.0),
            }
        }

        pub fn type_to_blood(&self)->f32{
            match self{
                Self::CommonZM=>300.0,
                Self::ConeHeadZM=>700.0,
                Self::PoleVaultingZM=>800.0,
            }
        }

        // pub fn type_to_x_offest(&self)->f32{
        //     match self{
        //         Self::CommonZM=>20.0,
        //         Self::ConeHeadZM=>10.0,
        //         Self::PoleVaultingZM=>10.0,
        //     }
        // }
    }

    impl TryFrom<usize> for ZombieType {
        type Error = &'static str;

        fn try_from(value: usize) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(ZombieType::CommonZM),
                1 => Ok(ZombieType::ConeHeadZM),
                2 => Ok(ZombieType::PoleVaultingZM),
                _ => Err("Invalid number for ZombieType"),
            }
        }
    }

    #[derive(PartialEq)]
    pub enum ZombieStatus{
        Walk0, //与初始状态一样walk
        Eat,
        Dead,
        // BloodLeftOver,
        Jump,
        Walk1, //撑杆僵尸跳过植物后的walk
    }

    impl ZombieStatus{
        pub fn status_to_index(&self)->usize{
            match self{
                Self::Walk0=>0,
                Self::Eat=>1,
                Self::Dead=>2,
                // Self::BloodLeftOver=>3,
                Self::Walk1=>3,
                Self::Jump=>4,
            }
        }

        pub fn status_to_frame_num(&self,zombie_type:&ZombieType)->usize{
            match zombie_type{
                ZombieType::CommonZM=>{
                    match self {
                        Self::Walk0=>22,
                        Self::Eat=>21,
                        Self::Dead=>10,
                        _=>0,
                    }
                },

                ZombieType::ConeHeadZM=>{
                    match self{
                        Self::Walk0=>20,
                        Self::Eat=>10,
                        Self::Dead=>10,
                        _=>0,
                    }
                },

                ZombieType::PoleVaultingZM=>{
                    match self{
                        Self::Walk0=>10,
                        Self::Eat=>14,
                        Self::Dead=>9,
                        Self::Walk1=>25,
                        Self::Jump=>16,
                    }
                }
            }
        }
    }

    //匹配状态
    // pub fn match_status()
}