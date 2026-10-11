//! Fenrir the Nordic Wolf
use super::prelude::*;

pub const ID: u32 = 91697229;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_AESIR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE | EFFECT_FLAG_SPSUM_PARAM, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_target_range(d, e1, POS_FACEUP_DEFENSE, 1);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    card::register_effect(d, c, e1, false);
    // self destroy
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_code(d, e4, EFFECT_SELF_DESTROY);
    effect::set_condition(d, e4, Some(lua_fn(descon)));
    card::register_effect(d, c, e4, false);
    // pos
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 1));
    effect::set_category(d, e5, CATEGORY_POSITION);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e5, EVENT_PHASE | PHASE_BATTLE_START);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    effect::set_condition(d, e5, Some(condition(poscon)));
    effect::set_target(d, e5, Some(cost_target(postg)));
    effect::set_operation(d, e5, Some(operation(posop)));
    card::register_effect(d, c, e5, false);
    // damage
    let e6 = effect::create_effect(d, c);
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e6, EFFECT_BOTH_BATTLE_DAMAGE);
    card::register_effect(d, c, e6, false);
}
// s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    match a.card(1) {
        // if c==nil then return Duel.IsPhase(PHASE_MAIN2) end
        None => duel::is_phase(d, PHASE_MAIN2),
        Some(c) => {
            // local tp=c:GetControler()
            let tp = card::get_controler(d, c);
            // return Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
            //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_AESIR),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
            let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_AESIR as u16]));
            duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && duel::is_existing_matching_card(d, Some(&f), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
        }
    }
}
// s.descon(e)
fn descon(d: &mut Duel, _a: Args) -> bool {
    // return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_AESIR),0,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_AESIR as u16]));
    !duel::is_existing_matching_card(d, Some(&f), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
// s.poscon
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
// s.postg
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(Card.IsDefensePos,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_defense_pos(d, c)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), n, 0, 0);
    true
}
// s.posop
fn posop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsDefensePos,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_defense_pos(d, c)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if #g>0 then Duel.ChangePosition(g,POS_FACEUP_ATTACK) end
    if !g.is_empty() {
        duel::change_position(d, g, POS_FACEUP_ATTACK as u8, None, None, None, false);
    }
}
