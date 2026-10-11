//! Generaider Boss Bite
use super::prelude::*;

pub const ID: u32 = 720147;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GENERAIDER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.check(sg,tp,exg)`
fn check(d: &mut Duel, sg: &CardSet, tp: u8, _exg: &CardSet) -> bool {
    // return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,sg,sg,tp)
    duel::is_existing_target(d, Some(&|d, c| filter(d, c, sg, tp)), tp, LOCATION_MZONE, 0, 1, sg.into())
}
/// `s.filter(c,sg,tp)`
fn filter(d: &mut Duel, c: CardId, sg: &CardSet, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ)
    //     and Duel.IsExistingMatchingCard(s.matfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,#sg,sg,c)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_XYZ)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, x| matfilter(d, x, c)),
            tp,
            LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE,
            0,
            sg.len() as u32,
            sg.into(),
        )
}
/// `s.matfilter(c,tc)`
fn matfilter(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    // return c:IsSetCard(SET_GENERAIDER) and c:IsMonster() and not c:IsType(TYPE_TOKEN)
    //     and (c:IsFaceup() or not c:IsLocation(LOCATION_MZONE)) and c~=tc
    card::is_set_card(d, c, &[SET_GENERAIDER as u16])
        && card::is_monster(d, c)
        && !card::is_type(d, c, TYPE_TOKEN)
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_MZONE))
        && c != tc
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsSetCard,1,false,s.check,nil,SET_GENERAIDER) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(
            d,
            tp,
            Some(&|d, c| card::is_set_card(d, c, &[SET_GENERAIDER as u16])),
            1,
            1,
            false,
            Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| (check(d, sg, tp, exg), false)),
            NO_EX,
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsSetCard,1,99,false,s.check,nil,SET_GENERAIDER)
    let g = aux::select_release_group_cost(
        d,
        tp,
        Some(&|d, c| card::is_set_card(d, c, &[SET_GENERAIDER as u16])),
        1,
        99,
        false,
        Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| (check(d, sg, tp, exg), false)),
        NO_EX,
    );
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // og:KeepAlive()
    // e:SetLabelObject(og)
    effect::set_label_object(d, a.e, Value::Group(og));
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc,g,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc, &g.clone().nil(), tp);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,g,tp)
    let gg = g.nil();
    duel::select_target(d, tp, Some(&|d, c| filter(d, c, &gg, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local rg=e:GetLabelObject()
    let rg = effect::get_label_object(d, a.e).as_group().cloned();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) and rg and #rg>0 then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        if let Some(rg) = rg.filter(|rg| !rg.is_empty()) {
            // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
            duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
            let n = rg.len() as u16;
            // local g=Duel.SelectMatchingCard(tp,s.matfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,#rg,#rg,rg,tc)
            let g = duel::select_matching_card(
                d,
                tp,
                Some(&|d, x| matfilter(d, x, tc)),
                tp,
                LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE,
                0,
                n,
                n,
                false,
                (&rg).into(),
            )
            .nil();
            // if #g>0 then
            if !g.is_empty() {
                // g:ForEach(Card.CancelToGrave)
                for x in group::iter(d, &g) {
                    card::cancel_to_grave(d, x, true);
                }
                // Duel.Overlay(tc,g,true)
                duel::overlay(d, tc, g, true);
            }
        }
    }
}
