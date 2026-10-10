//! Dogmatikamacabre
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 60921537;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[40352445, 48654323], listed_series: &[SET_DOGMATIKA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual Summon
    // local e1=Ritual.AddProcGreater({handler=c,filter=aux.FilterBoolFunction(Card.IsSetCard,SET_DOGMATIKA),
    //   location=LOCATION_HAND|LOCATION_GRAVE,extrafil=s.extramat,extratg=s.extratg,stage2=s.stage2})
    let e1 = ritual::add_proc_greater(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_GREATER,
            filter: Some(Arc::new(|d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_DOGMATIKA as u16]))),
            location: Some(LOCATION_HAND | LOCATION_GRAVE),
            extrafil: Some(Arc::new(|d: &mut Duel, a: &EffArgs| extramat(d, a))),
            extratg: Some(cost_target(extratg)),
            stage2: Some(Arc::new(|d: &mut Duel, mat: &CardSet, a: &EffArgs, tc: CardId| stage2(d, mat, a, tc))),
            ..Default::default()
        },
    );
    // e1:SetCategory(e1:GetCategory()+CATEGORY_TOGRAVE)
    let cat = effect::get_category(d, e1);
    effect::set_category(d, e1, cat + CATEGORY_TOGRAVE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.matfilter(c)
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:HasLevel() and c:IsType(TYPE_FUSION|TYPE_SYNCHRO) and c:IsAbleToRemove()
    card::has_level(d, c) && card::is_type(d, c, TYPE_FUSION | TYPE_SYNCHRO) && card::is_able_to_remove(d, c, None, None, None)
}
// function s.extramat(e,tp,eg,ep,ev,re,r,rp,chk)
fn extramat(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    // return Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION)
    //   and Group.NewGroup() or Duel.GetMatchingGroup(s.matfilter,tp,LOCATION_GRAVE,0,nil)
    if duel::is_player_affected_by_effect(d, a.tp, CARD_SPIRIT_ELIMINATION).is_some() {
        Some(CardSet::new())
    } else {
        Some(duel::get_matching_group(d, Some(&matfilter), a.tp, LOCATION_GRAVE, 0, NO_EX))
    }
}
// function s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,PLAYER_EITHER,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, PLAYER_EITHER as u8, LOCATION_EXTRA as i32);
    true
}
// function s.onfield(code)
fn onfield(d: &mut Duel, code: u32) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,code),0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[code])),
        0,
        LOCATION_ONFIELD,
        LOCATION_ONFIELD,
        1,
        NO_EX,
    )
}
// function s.stage2(mat,e,tp,eg,ep,ev,re,r,rp,tc)
fn stage2(d: &mut Duel, _mat: &CardSet, a: &EffArgs, _tc: CardId) {
    let tp = a.tp;
    // if not (s.onfield(40352445) and s.onfield(48654323)) then return end
    if !(onfield(d, 40352445) && onfield(d, 48654323)) {
        return;
    }
    // local g1=Duel.GetFieldGroup(tp,LOCATION_EXTRA,0)
    let g1 = duel::get_field_group(d, tp, LOCATION_EXTRA, 0);
    // local g2=Duel.GetFieldGroup(tp,0,LOCATION_EXTRA)
    let g2 = duel::get_field_group(d, tp, 0, LOCATION_EXTRA);
    // if (#g1<1 and #g2<1) or not Duel.SelectYesNo(tp,aux.Stringid(id,0)) then return end
    if (g1.is_empty() && g2.is_empty()) || !duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        return;
    }
    // local op=Duel.SelectEffect(tp,{#g1>0,aux.Stringid(id,1)},{#g2>0,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(!g1.is_empty(), aux::stringid(ID, 1)), (!g2.is_empty(), aux::stringid(ID, 2))]);
    // local g=(op==1) and g1 or g2
    let g = if op == Some(1) { g1.clone() } else { g2.clone() };
    // if op==2 then Duel.ConfirmCards(tp,g) end
    if op == Some(2) {
        duel::confirm_cards(d, tp, g.clone());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local tg=g:FilterSelect(tp,aux.AND(Card.IsMonster,Card.IsAbleToGrave),1,1,nil)
    let tg = group::filter_select(d, &g, tp, &|d, c| card::is_monster(d, c) && card::is_able_to_grave(d, c, None), 1, 1, false, NO_EX).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoGrave(tg,REASON_EFFECT)
        duel::send_to_grave(d, tg, REASON_EFFECT, None, None);
        // if op==2 then Duel.ShuffleExtra(1-tp) end
        if op == Some(2) {
            duel::shuffle_extra(d, 1 - tp);
        }
    }
}
