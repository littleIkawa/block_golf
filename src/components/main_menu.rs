use std::collections::HashMap;

use bevy::prelude::*;

use super::game::GameRule;

/// 各種オプションをゲームステートに受け渡すためのリソース
#[derive(Default)]
pub struct MenuOptionResource {
    pub current_layer: u32,
    pub current_option_num: u32,
    /// 現在のレイヤーまでに経由しているレイヤーを保持する
    pub layer_stack: Vec<u32>,
    /// レイヤー番号に対して最後にいた選択肢番号を保存しておく
    pub layer_choice_table: HashMap<u32, u32>,
}

/// メニューとして表示される選択肢セットレイヤー番号とidのセットを用いた識別子
#[derive(Component)]
pub struct MenuLayerPos(pub u32, pub u32);
pub struct ChangeMenuLayerEvent(pub u32);

pub struct MenuOptionSets {
    pub option_set: Vec<MenuOptionSet>,
}

pub struct MenuOptionSet {
    pub options: Vec<MenuOption>,
    pub layer_num: u32,
}

// TODO: enumをこれに変換するマクロとかあったらよさそう
pub struct MenuOption {
    /// 一つの選択肢セットの中で被っていなければいい
    pub id: u32,
    pub name: &'static str,
    pub disabled: bool,
}
impl MenuOption {
    pub fn new(name: &'static str, id: u32) -> Self {
        Self {
            id,
            name,
            disabled: false,
        }
    }
}
/// 階層ごとにエンティティの順番を保持したいのでこのようなリソースを作る
pub struct MenuLayerOptionEntities(pub HashMap<u32, Vec<Entity>>);

/// 選択肢のテキストであることを表す.
#[derive(Component)]
pub struct OptionText;
/// 選択中オプション
#[derive(Component)]
pub struct CurrentOption;
#[derive(Component)]
pub struct GameRuleOption(pub GameRule);
