//! El Dorado Adelantado
use super::prelude::*;

pub const ID: u32 = 92379223;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ELDLICH as u32, SET_ELDLIXIR as u32, SET_GOLDEN_LAND as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy cards or halve LP
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_ELDLICH),tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ELDLICH as u16])),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSpellTrap() and c:IsAbleToDeck()
    card::is_faceup(d, c) && card::is_spell_trap(d, c) && card::is_able_to_deck(d, c)
}
/// `Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_REMOVED,0,nil):Filter(Card.IsSetCard,nil,setcode)`
fn removed_set(d: &mut Duel, tp: u8, setcode: u32) -> CardSet {
    let g = duel::get_matching_group(d, Some(&tdfilter), tp, LOCATION_REMOVED, 0, NO_EX);
    group::filter(d, &g, &|d, x| card::is_set_card(d, x, &[setcode as u16]), NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g1=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_REMOVED,0,nil):Filter(Card.IsSetCard,nil,SET_ELDLIXIR)
    let g1 = removed_set(d, tp, SET_ELDLIXIR);
    // local g2=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_REMOVED,0,nil):Filter(Card.IsSetCard,nil,SET_GOLDEN_LAND)
    let g2 = removed_set(d, tp, SET_GOLDEN_LAND);
    let dn = aux::dncheck();
    // local b1=#g1>=3 and aux.SelectUnselectGroup(g1,e,tp,3,3,aux.dncheck,0)
    //     and Duel.IsExistingMatchingCard(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c)
    let b1 = g1.len() >= 3
        && matches!(
            aux::select_unselect_group(d, &g1, Some(a.e), tp, Some(3), Some(3), Some(&dn), 0, tp, None, None, None, false),
            Err(true)
        )
        && duel::is_existing_matching_card(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    // local b2=#g2>=3 and aux.SelectUnselectGroup(g2,e,tp,3,3,aux.dncheck,0)
    let b2 = g2.len() >= 3
        && matches!(
            aux::select_unselect_group(d, &g2, Some(a.e), tp, Some(3), Some(3), Some(&dn), 0, tp, None, None, None, false),
            Err(true)
        );
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,0)}, {b2,aux.Stringid(id,1)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 0)), (b2, aux::stringid(ID, 1))]);
    if op == Some(1) {
        // e:SetCategory(CATEGORY_TODECK+CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_TODECK + CATEGORY_DESTROY);
        // e:SetOperation(s.desop)
        effect::set_operation(d, a.e, Some(operation(desop)));
        // local dg=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,c)
        let dg = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,#dg,0,0)
        let n = dg.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, 0, 0);
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_TODECK+CATEGORY_RECOVER)
        effect::set_category(d, a.e, CATEGORY_TODECK + CATEGORY_RECOVER);
        // e:SetOperation(s.lpop)
        effect::set_operation(d, a.e, Some(operation(lpop)));
        // local rec=math.ceil(Duel.GetLP(1-tp)/2)
        let rec = ((duel::get_lp(d, 1 - tp) as f64) / 2.0).ceil() as i64;
        // Duel.SetTargetPlayer(tp)
        duel::set_target_player(d, tp);
        // Duel.SetTargetParam(rec)
        duel::set_target_param(d, rec as u32);
        // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,rec)
        duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, rec as i32);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,3,0,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 3, 0, LOCATION_REMOVED as i32);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_REMOVED,0,nil):Filter(Card.IsSetCard,nil,SET_ELDLIXIR)
    let g = removed_set(d, tp, SET_ELDLIXIR);
    // if #g<3 then return end
    if g.len() < 3 {
        return;
    }
    let dn = aux::dncheck();
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,aux.dncheck,1,tp,HINTMSG_TODECK)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(3), Some(3), Some(&dn), 1, tp, Some(HINTMSG_TODECK as u64), None, None, false)
        .unwrap_or_default();
    // if #sg==3 and Duel.SendtoDeck(sg,tp,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 then
    if sg.len() == 3 && duel::send_to_deck(d, &sg, Some(tp), SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 {
        // local dg=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,c)
        // (`c` is not a local of this function: it is the global nil, so nothing is excluded)
        let dg = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
fn lpop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_REMOVED,0,nil):Filter(Card.IsSetCard,nil,SET_GOLDEN_LAND)
    let g = removed_set(d, tp, SET_GOLDEN_LAND);
    // if #g<3 then return end
    if g.len() < 3 {
        return;
    }
    let dn = aux::dncheck();
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,aux.dncheck,1,tp,HINTMSG_TODECK)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(3), Some(3), Some(&dn), 1, tp, Some(HINTMSG_TODECK as u64), None, None, false)
        .unwrap_or_default();
    // if #sg==3 and Duel.SendtoDeck(sg,tp,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 then
    if sg.len() == 3 && duel::send_to_deck(d, &sg, Some(tp), SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 {
        // local initial=Duel.GetLP(1-tp)
        let initial = duel::get_lp(d, 1 - tp);
        // local lp=math.ceil(initial/2)
        let lp = ((initial as f64) / 2.0).ceil() as i32;
        // Duel.SetLP(1-tp,lp)
        duel::set_lp(d, 1 - tp, lp);
        // local rec=Duel.GetLP(1-tp)
        let rec = duel::get_lp(d, 1 - tp);
        // if rec~=initial then
        if rec != initial {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Recover(tp,rec,REASON_EFFECT)
            duel::recover(d, tp, rec as i64, REASON_EFFECT, false, None);
        }
    }
}
