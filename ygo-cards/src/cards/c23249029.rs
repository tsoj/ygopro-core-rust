//! Cursed Copycat Noble Arms
use super::prelude::*;

pub const ID: u32 = 23249029;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Equipped monster gains 200 ATK/DEF for each Equip Spell in your field and GY
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1a, EFFECT_TYPE_EQUIP);
    // e1a:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1a, EFFECT_UPDATE_ATTACK);
    // e1a:SetValue(s.atkdefval)
    effect::set_value_fn(d, e1a, effect_card_value(atkdefval));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e1b, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Special Summon 1 Warrior monster with the same Attribute and Level as the equipped monster but a different name from your Deck, ...
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) local ec=e:GetHandler():GetEquipTarget() return ec and ec:IsRace(RACE_WARRIOR) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) local ec=e:GetHandler():GetEquipTarget() return ec and ec:IsRace(RACE_WARRIOR) end`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    match card::get_equip_target(d, h) {
        Some(ec) => card::is_race(d, ec, RACE_WARRIOR),
        None => false,
    }
}
/// `s.atkdefvalfilter(c)`
fn atkdefvalfilter(d: &mut Duel, c: CardId) -> bool {
    // c:IsEquipSpell() and (c:IsFaceup() or c:GetEquipTarget())
    card::is_equip_spell(d, c) && (card::is_faceup(d, c) || card::get_equip_target(d, c).is_some())
}
/// `s.atkdefval(e,c)`
fn atkdefval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return 200*Duel.GetMatchingGroupCount(s.atkdefvalfilter,e:GetHandlerPlayer(),LOCATION_ONFIELD|LOCATION_GRAVE,0,nil)
    let hp = effect::get_handler_player(d, e);
    200 * duel::get_matching_group_count(d, Some(&atkdefvalfilter), hp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, NO_EX) as i64
}
/// `s.spfilter(c,e,tp,attr,lv,code,hc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, attr: u32, lv: u32, code: u32, hc: CardId) -> bool {
    // c:IsRace(RACE_WARRIOR) and c:IsAttribute(attr) and c:IsLevel(lv) and not c:IsCode(code)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and hc:CheckEquipTarget(c)
    card::is_race(d, c, RACE_WARRIOR)
        && card::is_attribute(d, c, attr)
        && card::is_level(d, c, &[lv])
        && !card::is_code(d, c, &[code])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::check_equip_target(d, hc, c)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetEquipTarget()
    let ec = card::get_equip_target(d, c);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,ec:GetAttribute(),ec:GetLevel(),ec:GetCode(),c) end
    if a.chk == Some(0) {
        let (ct, _) = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None);
        if ct <= 0 {
            return false;
        }
        let ec = ec.nil();
        let attr = card::get_attribute(d, ec);
        let lv = card::get_level(d, ec);
        let code = card::get_code(d, ec)[0];
        return duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp, attr, lv, code, c)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,ec,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, ec.map(CardSet::from_card), 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // You cannot Special Summon for the rest of this turn, except Warrior monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return not c:IsRace(RACE_WARRIOR) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_race(d, c, RACE_WARRIOR))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    let (ct, _) = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None);
    if ct <= 0 {
        return;
    }
    // local ec=c:GetEquipTarget()
    // if not ec then return end
    let Some(ec) = card::get_equip_target(d, c) else { return };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,ec:GetAttribute(),ec:GetLevel(),ec:GetCode(),c):GetFirst()
    let attr = card::get_attribute(d, ec);
    let lv = card::get_level(d, ec);
    let code = card::get_code(d, ec)[0];
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp, attr, lv, code, c)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0
    //     and Duel.Equip(tp,c,sc) and ec:IsLocation(LOCATION_MZONE) then
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
            && duel::equip(d, tp, c, sc, true, false)
            && card::is_location(d, ec, LOCATION_MZONE)
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(ec,REASON_EFFECT)
            duel::destroy(d, ec, REASON_EFFECT, None, None);
        }
    }
}
