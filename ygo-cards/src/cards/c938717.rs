//! Boot-Up Corporal - Command Dynamo
use super::prelude::*;

pub const ID: u32 = 938717;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GADGET], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon itself from hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gains 1000 ATK per monster equipped by its effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.eqfilter(c)`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsSetCard(SET_GADGET) and c:IsFaceup() and c:IsMonster()
    card::is_race(d, c, RACE_MACHINE) && card::is_set_card(d, c, &[SET_GADGET as u16]) && card::is_faceup(d, c) && card::is_monster(d, c)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local g=Duel.GetMatchingGroup(s.eqfilter,tp,LOCATION_GRAVE|LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&eqfilter), tp, LOCATION_GRAVE | LOCATION_MZONE, 0, NO_EX);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE|LOCATION_MZONE) and chkc:IsControler(tp) and s.eqfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_MZONE) && card::is_controler(d, chkc, tp) && eqfilter(d, chkc);
    }
    // local ft=math.min(Duel.GetLocationCount(tp,LOCATION_SZONE),2)
    let ft = std::cmp::min(duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0, 2);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and ft>0
    //     and aux.SelectUnselectGroup(g,e,tp,1,ft,aux.dncheck,chk)
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && ft > 0
            && matches!(
                aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(ft as usize), Some(&aux::dncheck()), 0, tp, None, None, None, false),
                Err(true)
            )
            && card::is_can_be_special_summoned(d, h, e, 0, tp, false, false, None, None, None);
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,1,ft,aux.dncheck,1,tp,HINTMSG_EQUIP)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(ft as usize), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_EQUIP as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g), n, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0 {
        // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
        let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
        // local g=Duel.GetTargetCards(e):Match(Card.IsMonster,nil):Match(Card.IsSetCard,nil,SET_GADGET)
        let tg = duel::get_target_cards(d, a.e).nil();
        let g = group::filter(d, &tg, &|d, x| card::is_monster(d, x), NO_EX);
        let g = group::filter(d, &g, &|d, x| card::is_set_card(d, x, &[SET_GADGET as u16]), NO_EX);
        // if ft<#g then return end
        if (ft as i64) < g.len() as i64 {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // for tc in aux.Next(g) do
        for tc in group::iter(d, &g) {
            // Duel.Equip(tp,tc,c,false)
            duel::equip(d, tp, tc, c, false, false);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_OWNER_RELATE)
            effect::set_property(d, e1, EFFECT_FLAG_OWNER_RELATE, 0);
            // e1:SetCode(EFFECT_EQUIP_LIMIT)
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetValue(s.eqlimit)
            effect::set_value_fn(d, e1, effect_card_value(eqlimit));
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        }
    }
}
/// `s.eqlimit(e,c)`: `return e:GetOwner()==c`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    (effect::get_owner(d, e) == Some(c)) as i64
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    card::get_flag_effect(d, c, ID) != 0
}
/// `s.atkval(e,c)`: `return c:GetEquipGroup():FilterCount(s.atkfilter,nil)*1000`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let eg = card::get_equip_group(d, c);
    group::filter_count(d, &eg, &atkfilter, NO_EX) as i64 * 1000
}
