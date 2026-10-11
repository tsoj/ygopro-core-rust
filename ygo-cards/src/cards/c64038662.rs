//! Variable Form
use super::prelude::*;

pub const ID: u32 = 64038662;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INZEKTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // equip or spsummon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.eftg2)
    effect::set_target(d, e2, Some(cost_target(eftg2)));
    // e2:SetOperation(s.efop)
    effect::set_operation(d, e2, Some(operation(efop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_INZEKTOR)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_INZEKTOR as u16])
}
/// `s.filter2(c,e,tp)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_INZEKTOR) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_INZEKTOR as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.select(e,tp,b1,b2)`
fn select(d: &mut Duel, e: EffectId, tp: u8, b1: bool, b2: bool) {
    // local op=0
    // if b1 and b2 then
    //     op=Duel.SelectOption(tp,aux.Stringid(id,2),aux.Stringid(id,3))+1
    // elseif b1 then
    //     op=Duel.SelectOption(tp,aux.Stringid(id,2))+1
    // else op=Duel.SelectOption(tp,aux.Stringid(id,3))+2 end
    let op: i64 = if b1 && b2 {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 2), aux::stringid(ID, 3)]) as i64 + 1
    } else if b1 {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 2)]) as i64 + 1
    } else {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 3)]) as i64 + 2
    };
    // e:SetLabel(op)
    effect::set_label(d, e, &[op]);
    if op == 1 {
        // e:SetCategory(CATEGORY_EQUIP)
        effect::set_category(d, e, CATEGORY_EQUIP);
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,4))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 4));
        // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil)
        let g1 = duel::select_target(d, tp, Some(&filter1), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
        // local g2=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,g1:GetFirst())
        let first = group::get_first(d, &g1).nil();
        let _g2 = duel::select_target(d, tp, Some(&filter1), tp, LOCATION_MZONE, 0, 1, 1, false, first.into());
        // e:SetLabelObject(g1:GetFirst())
        effect::set_label_object(d, e, Value::Card(first));
    } else {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_SZONE,0,1,1,nil,e,tp)
        let g = duel::select_target(d, tp, Some(&|d, c| filter2(d, c, e, tp)), tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    }
}
/// `s.eftg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eftg2(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // if e:GetLabel()==0 or e:GetLabel()==1 then return false
        // else return chkc:IsLocation(LOCATION_SZONE) and chkc:IsControler(tp) and s.filter2(chkc,e,tp) end
        let label = effect::get_label(d, e);
        if label == 0 || label == 1 {
            return false;
        }
        return card::is_location(d, chkc, LOCATION_SZONE) && card::is_controler(d, chkc, tp) && filter2(d, chkc, e, tp);
    }
    // local b1=Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,2,nil) and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    let b1 = duel::is_existing_target(d, Some(&filter1), tp, LOCATION_MZONE, 0, 2, NO_EX)
        && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    // local b2=Duel.IsExistingTarget(s.filter2,tp,LOCATION_SZONE,0,1,nil,e,tp) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let b2 = duel::is_existing_target(d, Some(&|d, c| filter2(d, c, e, tp)), tp, LOCATION_SZONE, 0, 1, NO_EX)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // if chk==0 then return (b1 or b2) end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // s.select(e,tp,b1,b2)
    select(d, e, tp, b1, b2);
    true
}
/// `s.efop(e,tp,eg,ep,ev,re,r,rp)`
fn efop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    let label = effect::get_label(d, e);
    // if e:GetLabel()==0 then return
    if label == 0 {
        return;
    } else if label == 1 {
        // local tc1=e:GetLabelObject()
        let tc1 = effect::get_label_object(d, e).as_card().nil();
        // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
        let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])
            .first()
            .and_then(|v| v.as_group().cloned())
            .nil();
        // local tc2=g:GetFirst()
        let mut tc2 = group::get_first(d, &g);
        // if tc1==tc2 then tc2=g:GetNext() end
        if tc2 == Some(tc1) {
            tc2 = group::iter(d, &g).get(1).copied();
        }
        let tc2 = tc2.nil();
        // if tc1:IsFaceup() and tc2:IsFaceup() and tc1:IsRelateToEffect(e) and tc2:IsRelateToEffect(e) and Duel.Equip(tp,tc1,tc2,false) then
        if card::is_faceup(d, tc1)
            && card::is_faceup(d, tc2)
            && card::is_relate_to_effect(d, tc1, e)
            && card::is_relate_to_effect(d, tc2, e)
            && duel::equip(d, tp, tc1, tc2, false, false)
        {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_EQUIP_LIMIT)
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetValue(s.eqlimit)
            effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
            // e1:SetLabelObject(tc2)
            effect::set_label_object(d, e1, Value::Card(tc2));
            // tc1:RegisterEffect(e1)
            card::register_effect(d, tc1, e1, false);
        }
    } else {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d);
        // if tc and tc:IsRelateToEffect(e) then
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, e) {
                // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
                duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
            }
        }
    }
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetLabelObject()
    effect::get_label_object(d, e).as_card() == Some(c)
}
