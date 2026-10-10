//! Dragonecro Nethersoul Dragon
use super::prelude::*;

pub const ID: u32 = 8198620;
const TOKEN_DARK_SOUL: u32 = ID + 1;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, TOKEN_DARK_SOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 2 Zombie monsters
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsRace,RACE_ZOMBIE),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_ZOMBIE, Some(fc), st, Some(tp)))), 2)],
    );
    // c:AddMustFirstBeFusionSummoned()
    card::add_must_first_be_fusion_summoned(d, c);
    // You can only control 1 "Dragonecro Nethersoul Dragon"
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Monsters cannot be destroyed by battle with this card
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e1, Some(effect_card_fn(|d, e, c| Some(c) == card::get_battle_target(d, effect::get_handler(d, e).nil()))));
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // Change the ATK of a monster this card battles to 0 and Special Summon 1 "Dark Soul Token"
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    effect::set_target(d, e2, Some(cost_target(atktg)));
    effect::set_operation(d, e2, Some(operation(atkop)));
    card::register_effect(d, c, e2, false);
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h);
    if a.chk == Some(0) {
        return match bc {
            Some(bc) => card::is_relate_to_battle(d, bc),
            None => false,
        };
    }
    let bc = bc.nil();
    duel::set_target_card(d, bc);
    if card::get_original_level(d, bc) > 0 {
        duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    }
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let bc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, bc, a.e) && card::is_faceup(d, bc) {
        let c = effect::get_handler(d, a.e).nil();
        let atk = card::get_base_attack(d, bc);
        // Change that monster's ATK to 0
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_value(d, e1, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, bc, e1, false);
        let lv = card::get_original_level(d, bc);
        if lv > 0
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(
                d,
                a.tp,
                TOKEN_DARK_SOUL,
                duel::MonsterData {
                    setcodes: Some(vec![0]),
                    type_: Some(TYPES_TOKEN),
                    attack: Some(atk),
                    defense: Some(0),
                    level: Some(lv),
                    race: Some(RACE_ZOMBIE),
                    attribute: Some(ATTRIBUTE_DARK),
                },
                Some(POS_FACEUP as u8),
                Some(a.tp),
                0,
            )
        {
            let token = duel::create_token(d, a.tp, TOKEN_DARK_SOUL).nil();
            card::info_level(d, token, Some(lv));
            if duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
                // Set the Token's original ATK to the original ATK of the monster that this card battled
                let e1 = effect::create_effect(d, c);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
                effect::set_value(d, e1, atk as i64);
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                card::register_effect(d, token, e1, false);
            }
            duel::special_summon_complete(d);
        }
    }
}
