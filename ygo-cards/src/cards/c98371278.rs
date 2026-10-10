//! DMZ Dragon
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 98371278;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Once per turn: You can target 1 Level 4 or lower Dragon monster in your GY and 1 Dragon monster you control; ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // At the end of the Damage Step, ...
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsRace(RACE_DRAGON) and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_level_below(d, c, 4) && card::is_race(d, c, RACE_DRAGON) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
/// `aux.FaceupFilter(Card.IsRace,RACE_DRAGON)`
fn faceup_dragon(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_DRAGON)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.eqfilter,tp,LOCATION_GRAVE,0,1,nil,tp)
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsRace,RACE_DRAGON),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| eqfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&faceup_dragon), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.eqfilter,tp,LOCATION_GRAVE,0,1,1,nil,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| eqfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsRace,RACE_DRAGON),tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&faceup_dragon), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g, 1, tp, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tg=Duel.GetTargetCards(e):Match(Card.IsFaceup,nil)
    let mut tg = duel::get_target_cards(d, a.e).nil();
    group::match_(d, &mut tg, &|d, c| card::is_faceup(d, c), NO_EX);
    // if #tg~=2 then return end
    if tg.len() != 2 {
        return;
    }
    // local tc=tg:Filter(Card.IsLocation,nil,LOCATION_MZONE):GetFirst()
    let mzone = group::filter(d, &tg, &|d, c| card::is_location(d, c, LOCATION_MZONE), NO_EX);
    let tc = group::get_first(d, &mzone).nil();
    // local ec=(tg-tc):GetFirst()
    let mut rest = tg.clone();
    group::remove_card(&mut rest, &CardSet::from_card(tc));
    let ec = group::get_first(d, &rest).nil();
    // if Duel.Equip(tp,ec,tc) then
    if duel::equip(d, tp, ec, tc, true, false) {
        // local e1=Effect.CreateEffect(ec)
        let e1 = effect::create_effect(d, ec);
        // e1:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(500)
        effect::set_value(d, e1, 500);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // ec:RegisterEffect(e1)
        card::register_effect(d, ec, e1, false);
        // local e2=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e2 = effect::create_effect(d, h);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
        // e2:SetValue(function(e,c) return c==tc end)
        effect::set_value_fn(d, e2, effect_card_fn(move |_d, _e, c| c == tc));
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // ec:RegisterEffect(e2)
        card::register_effect(d, ec, e2, false);
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=Duel.GetAttacker()
    let bc = duel::get_attacker(d);
    // return bc and bc:IsControler(tp) and bc:HasEquipCard()
    match bc {
        Some(bc) => card::is_controler(d, bc, a.tp) && card::has_equip_card(d, bc),
        None => false,
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local equip_group=Duel.GetAttacker():GetEquipGroup()
    let bc = duel::get_attacker(d).nil();
    let equip_group = card::get_equip_group(d, bc);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,equip_group,#equip_group,tp,0)
    let n = equip_group.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(equip_group), n, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local bc=Duel.GetAttacker()
    let bc = duel::get_attacker(d).nil();
    // if bc:IsRelateToBattle() and bc:IsControler(tp) and bc:HasEquipCard()
    //     and Duel.Destroy(bc:GetEquipGroup(),REASON_EFFECT)>0 then
    if card::is_relate_to_battle(d, bc) && card::is_controler(d, bc, a.tp) && card::has_equip_card(d, bc) {
        let eg = card::get_equip_group(d, bc);
        if duel::destroy(d, eg, REASON_EFFECT, None, None) > 0 {
            // Duel.ChainAttack()
            duel::chain_attack(d, None);
        }
    }
}
