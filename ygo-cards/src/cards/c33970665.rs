//! Guts of Steel
use super::prelude::*;

pub const ID: u32 = 33970665;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SCRAP], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SCRAP) and c:IsAbleToRemove()
    //     and (c:IsCanBeSpecialSummoned(e,0,tp,false,false) or c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp))
    card::is_set_card(d, c, &[SET_SCRAP as u16])
        && card::is_able_to_remove(d, c, None, None, None)
        && (card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
            || card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter(d, chkc, e, tp);
    }
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION)
    //     and (Duel.GetLocationCount(tp,LOCATION_MZONE)>0 or Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0)
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,3,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none()
            && (duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                || duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0)
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 3, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,3,3,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 3, 3, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetTargetCards(e)
    let mut g = duel::get_target_cards(d, e).nil();
    // local ft1=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local ft2=Duel.GetLocationCount(1-tp,LOCATION_MZONE)
    let ft2 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0;
    // if #g==0 or (ft1==0 and ft2==0) then return end
    if g.is_empty() || (ft1 == 0 && ft2 == 0) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_SPSUMMON as u64);
    // local tc=g:Select(1-tp,1,1,nil):GetFirst()
    let sel = group::select(d, &g, 1 - tp, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // local s1=ft1>0 and tc:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let s1 = ft1 > 0 && card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None);
    // local s2=ft2>0 and tc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)
    let s2 = ft2 > 0 && card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None);
    // if s1 and s2 then op=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
    // elseif s1 then op=Duel.SelectOption(tp,aux.Stringid(id,0))
    // elseif s2 then op=Duel.SelectOption(tp,aux.Stringid(id,1))+1
    // else op=2 end
    let op = if s1 && s2 {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)])
    } else if s1 {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 0)])
    } else if s2 {
        duel::select_option(d, tp, true, &[aux::stringid(ID, 1)]) + 1
    } else {
        2
    };
    // if op==0 then Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
    if op == 0 {
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
    // elseif op==1 then Duel.SpecialSummon(tc,0,tp,1-tp,false,false,POS_FACEUP) end
    else if op == 1 {
        duel::special_summon(d, tc, 0, tp, 1 - tp, false, false, POS_FACEUP as u8, None);
    }
    // g:RemoveCard(tc)
    group::remove_card(&mut g, &CardSet::from_card(tc));
    // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
