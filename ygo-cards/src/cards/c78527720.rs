//! Prinzessin
use super::prelude::*;

pub const ID: u32 = 78527720;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[14512825, CARD_STROMBERG, 9677699], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Pumpkin Carriage from hand or Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Equip "Glass Slippers" to another monster on the field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e3, EVENT_BATTLE_DAMAGE);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e3:SetCondition(s.eqcon)
    effect::set_condition(d, e3, Some(condition(eqcon)));
    // e3:SetTarget(s.eqtg)
    effect::set_target(d, e3, Some(cost_target(eqtg)));
    // e3:SetOperation(s.eqop)
    effect::set_operation(d, e3, Some(operation(eqop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(14512825) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[14512825]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.eqfilter1(c,ec)`
fn eqfilter1(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return c:IsCode(9677699) and c:CheckEquipTarget(ec) and not c:IsForbidden()
    card::is_code(d, c, &[9677699]) && card::check_equip_target(d, c, ec) && !card::is_forbidden(d, c)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // if c:IsRelateToEffect(e) and c:IsFaceup() and c:IsControler(tp) and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
            //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_STROMBERG),tp,LOCATION_FZONE,LOCATION_FZONE,1,nil,tp)
            //     and Duel.IsExistingMatchingCard(s.eqfilter1,tp,LOCATION_DECK,0,1,nil,e:GetHandler())
            //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            if card::is_relate_to_effect(d, c, a.e)
                && card::is_faceup(d, c)
                && card::is_controler(d, c, a.tp)
                && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
                && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_code(d, x, &[CARD_STROMBERG]))), a.tp, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
                && duel::is_existing_matching_card(d, Some(&|d, x| eqfilter1(d, x, c)), a.tp, LOCATION_DECK, 0, 1, NO_EX)
                && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
            {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
                // local g2=Duel.SelectMatchingCard(tp,s.eqfilter1,tp,LOCATION_DECK,0,1,1,nil,c)
                let g2 = duel::select_matching_card(d, a.tp, Some(&|d, x| eqfilter1(d, x, c)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
                // if #g2>0 then
                if group::get_count(&g2) > 0 {
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.Equip(tp,g2:GetFirst(),c)
                    let eq = group::get_first(d, &g2).nil();
                    duel::equip(d, a.tp, eq, c, true, false);
                }
            }
        }
    }
}
/// `s.eqcon(e,tp,eg,ep,ev,re,r,rp)`
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and Duel.GetAttackTarget()==nil
    a.ep == 1 - a.tp && duel::get_attack_target(d).is_none()
}
/// `s.eqfilter2(c,tc,tp)`
fn eqfilter2(d: &mut Duel, c: CardId, tc: CardId, tp: u8) -> bool {
    // return c:IsCode(9677699) and tc:GetEquipGroup():IsContains(c)
    //     and Duel.IsExistingTarget(s.eqfilter3,tp,LOCATION_MZONE,LOCATION_MZONE,1,tc,c)
    //     and c:CheckUniqueOnField(tp) and not c:IsForbidden()
    if !card::is_code(d, c, &[9677699]) {
        return false;
    }
    let eg = card::get_equip_group(d, tc);
    if !group::is_contains(&eg, c) {
        return false;
    }
    duel::is_existing_target(d, Some(&|d, x| eqfilter3(d, x, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, tc.into())
        && card::check_unique_on_field(d, c, tp, None, None)
        && !card::is_forbidden(d, c)
}
/// `s.eqfilter3(c,ec)`
fn eqfilter3(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return c:IsFaceup() and ec:CheckEquipTarget(c)
    card::is_faceup(d, c) && card::check_equip_target(d, ec, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.eqfilter2,tp,LOCATION_SZONE,0,1,nil,c,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| eqfilter2(d, x, c, a.tp)), a.tp, LOCATION_SZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g1=Duel.SelectTarget(tp,s.eqfilter2,tp,LOCATION_SZONE,0,1,1,nil,c,tp)
    let g1 = duel::select_target(d, a.tp, Some(&|d, x| eqfilter2(d, x, c, a.tp)), a.tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX);
    let g1_first = group::get_first(d, &g1.clone().nil());
    // e:SetLabelObject(g1:GetFirst())
    effect::set_label_object(d, a.e, Value::card(g1_first));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g2=Duel.SelectTarget(tp,s.eqfilter3,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,c,g1:GetFirst())
    let _g2 = duel::select_target(d, a.tp, Some(&|d, x| eqfilter3(d, x, g1_first.nil())), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g1,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g1, 1, a.tp, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local ec=e:GetLabelObject()
    let ec = effect::get_label_object(d, a.e).as_card().nil();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local tc=g:GetFirst()
    // if tc==ec then tc=g:GetNext() end
    let order = group::iter(d, &g);
    let mut tc = order.first().copied();
    if tc == Some(ec) {
        tc = order.get(1).copied();
    }
    // if ec:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, ec, a.e) {
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // Duel.Equip(tp,ec,tc)
            duel::equip(d, a.tp, ec, tc, true, false);
        }
    }
}
