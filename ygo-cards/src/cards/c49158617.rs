//! Dual Avatar Defeating Evil
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 49158617;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_DUAL_AVATAR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DRAW+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DRAW + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `aux.FaceupFilter(Card.IsSetCard,SET_DUAL_AVATAR)`
fn faceup_dual_avatar(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DUAL_AVATAR as u16])
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsSetCard,SET_DUAL_AVATAR),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(aux.TRUE,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&faceup_dual_avatar), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g1=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsSetCard,SET_DUAL_AVATAR),tp,LOCATION_MZONE,0,1,1,nil)
    let mut g1 = duel::select_target(d, tp, Some(&faceup_dual_avatar), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g2=Duel.SelectTarget(tp,aux.TRUE,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g2 = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g1,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g1), 2, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,1,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 1, tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, LOCATION_GRAVE as i32);
    true
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DUAL_AVATAR) and c:IsType(TYPE_FUSION) and c:IsPreviousControler(tp)
    card::is_set_card(d, c, &[SET_DUAL_AVATAR as u16]) && card::is_type(d, c, TYPE_FUSION) && card::is_previous_controler(d, c, tp)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local tg=g:Filter(Card.IsRelateToEffect,nil,e)
    let tg = group::filter(d, &g, &|d, c| card::is_relate_to_effect(d, c, e), NO_EX);
    // if #tg>0 and Duel.Destroy(tg,REASON_EFFECT)>0 then
    if !tg.is_empty() && duel::destroy(d, tg, REASON_EFFECT, None, None) > 0 {
        // local og=Duel.GetOperatedGroup():Filter(s.filter,nil,tp)
        let operated = duel::get_operated_group(d);
        let og = group::filter(d, &operated, &|d, c| filter(d, c, tp), NO_EX);
        // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_GRAVE,nil)
        let gr = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_GRAVE, NO_EX);
        // local opt1=Duel.IsPlayerCanDraw(tp,1)
        let opt1 = duel::is_player_can_draw(d, tp, 1);
        // local opt2=#g>0
        let opt2 = !gr.is_empty();
        // if #og>0 and (opt1 or opt2) and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
        if !og.is_empty() && (opt1 || opt2) && duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
            // local opt
            // if opt1 and opt2 then opt=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
            // elseif opt1 and not opt2 then opt=Duel.SelectOption(tp,aux.Stringid(id,1))
            // elseif opt2 and not opt1 then opt=Duel.SelectOption(tp,aux.Stringid(id,2))+1 end
            let opt: i32 = if opt1 && opt2 {
                duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)])
            } else if opt1 {
                duel::select_option(d, tp, true, &[aux::stringid(ID, 1)])
            } else {
                duel::select_option(d, tp, true, &[aux::stringid(ID, 2)]) + 1
            };
            // if opt==0 then
            if opt == 0 {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Draw(tp,1,REASON_EFFECT)
                duel::draw(d, tp, 1, REASON_EFFECT);
            // elseif opt==1 then
            } else if opt == 1 {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
                // local sg=g:Select(tp,1,1,nil)
                let sg = group::select(d, &gr, tp, 1, 1, false, NO_EX).nil();
                // Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)
                duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
            }
        }
    }
}
