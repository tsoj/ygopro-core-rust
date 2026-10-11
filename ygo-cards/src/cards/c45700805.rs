//! Plasma Warrior Eitom
use super::prelude::*;

pub const ID: u32 = 45700805;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_MAGNET_WARRIOR, SET_MAGNA_WARRIOR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // Must be Special Summoned (from your hand or GY) by Tributing 1 Level 7 or higher monster. ...
    let e0 = effect::create_effect(d, c);
    effect::set_description(d, e0, aux::stringid(ID, 0));
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e0, LOCATION_HAND | LOCATION_GRAVE);
    effect::set_count_limit(d, e0, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e0, Some(lua_fn(spcon)));
    effect::set_target(d, e0, Some(lua_fn(sptg)));
    effect::set_operation(d, e0, Some(lua_fn(spop)));
    card::register_effect(d, c, e0, false);
    // Once per turn: You can send 1 "Magnet Warrior" or "Magna Warrior" monster from your Deck to the GY, ...
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
}
/// `function(e) local c=e:GetHandler() return not (c:IsBaseAttack(1500) and c:IsHasEffect(EFFECT_DIRECT_ATTACK)) end`
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    !(card::is_base_attack(d, c, 1500) && !card::is_has_effect(d, c, EFFECT_DIRECT_ATTACK, None).is_empty())
}
/// `function(e,c) if c==nil then return true end ... end` (the Special Summon procedure's condition)
fn spcon(d: &mut Duel, a: Args) -> bool {
    let c = match a.card(1) {
        Some(c) => c,
        None => return true,
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.CheckReleaseGroup(tp,Card.IsLevelAbove,1,false,1,true,c,tp,nil,false,nil,7)
    let ra = duel::ReleaseArgs {
        use_hand: Some(false),
        max: Some(1),
        check_field: true,
        to_check: Some(c),
        toplayer: Some(tp),
        zone: None,
        use_oppo: false,
        cancelable: false,
    };
    duel::check_release_group(d, tp, Some(&|d, x| card::is_level_above(d, x, 7)), 1, ra, NO_EX, false)
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk,c)` (the Special Summon procedure's target)
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.c(9);
    // local g=Duel.SelectReleaseGroup(tp,Card.IsLevelAbove,1,1,false,true,true,c,nil,nil,false,nil,7)
    let ra = duel::ReleaseArgs {
        use_hand: Some(false),
        max: None,
        check_field: true,
        to_check: Some(c),
        toplayer: None,
        zone: None,
        use_oppo: false,
        cancelable: true,
    };
    let g = duel::select_release_group(d, tp, Some(&|d, x| card::is_level_above(d, x, 7)), 1, 1, ra, NO_EX, false);
    // if g and #g>0 then
    match g {
        Some(g) if !g.is_empty() => {
            // e:SetLabelObject(g)
            effect::set_label_object(d, e, Value::Group(g));
            true
        }
        _ => false,
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp,c)` (the Special Summon procedure's operation)
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if g then Duel.Release(g,REASON_COST) end
    if let Some(g) = effect::get_label_object(d, e).as_group().cloned() {
        duel::release(d, g, REASON_COST, None);
    }
}
/// `s.atkcostfilter(c)`
fn atkcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_MAGNET_WARRIOR,SET_MAGNA_WARRIOR}) and c:IsMonster() and not c:IsCode(id) and c:IsAbleToGraveAsCost()
    card::is_set_card(d, c, &[SET_MAGNET_WARRIOR as u16, SET_MAGNA_WARRIOR as u16])
        && card::is_monster(d, c)
        && !card::is_code(d, c, &[ID])
        && card::is_able_to_grave_as_cost(d, c)
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.atkcostfilter,tp,LOCATION_DECK,0,1,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&atkcostfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.atkcostfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&atkcostfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // if c:IsFaceup() then
        if card::is_faceup(d, c) {
            // For the rest of this turn, this card's original ATK becomes 1500
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_BASE_ATTACK)
            effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
            // e1:SetValue(1500)
            effect::set_value(d, e1, 1500);
            // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
        // Also it can attack directly
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetDescription(3205)
        effect::set_description(d, e2, 3205);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CLIENT_HINT, 0);
        // e2:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
