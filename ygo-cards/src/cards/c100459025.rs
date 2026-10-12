//! Final Journey of the Soul
use super::prelude::*;

pub const ID: u32 = 100459025;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_SPECIAL_SUMMON+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local ex1,g1,gc1,dp1,loc1=Duel.GetOperationInfo(ev,CATEGORY_SPECIAL_SUMMON)
    let op1 = duel::get_operation_info(d, a.ev, CATEGORY_SPECIAL_SUMMON);
    // local ex2,g2,gc2,dp2,loc2=Duel.GetPossibleOperationInfo(ev,CATEGORY_SPECIAL_SUMMON)
    let op2 = duel::get_possible_operation_info(d, a.ev, CATEGORY_SPECIAL_SUMMON);
    // if not (ex1 or ex2) then return false end
    if op1.is_none() && op2.is_none() {
        return false;
    }
    // local g=Group.CreateGroup()
    let mut g = CardSet::new();
    // if g1 then g:Merge(g1) end
    // if g2 then g:Merge(g2) end
    for op in [&op1, &op2].into_iter().flatten() {
        if let Some(cards) = &op.op_cards {
            group::add_card(&mut g, cards);
        }
    }
    // return (((loc1 or 0)|(loc2 or 0))&LOCATION_GRAVE)>0
    //     or (#g>0 and g:IsExists(function(c) return c:IsLocation(LOCATION_GRAVE) and c:IsMonster() end,1,nil))
    let loc1 = op1.as_ref().map_or(0, |op| op.op_param);
    let loc2 = op2.as_ref().map_or(0, |op| op.op_param);
    ((loc1 | loc2) & LOCATION_GRAVE as i32) > 0
        || (!g.is_empty() && group::is_exists(d, &g, &|d, x| card::is_location(d, x, LOCATION_GRAVE) && card::is_monster(d, x), 1, NO_EX))
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,0,LOCATION_GRAVE,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,1-tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - tp, LOCATION_GRAVE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - tp, LOCATION_MZONE as i32);
    true
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // local atk_total=sg:GetSum(Card.GetAttack)
    let atk_total = group::get_sum(d, sg, &|d, x| card::get_attack(d, x) as i64);
    // return atk_total<=6000,atk_total>6000
    (atk_total <= 6000, atk_total > 6000)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.NegateEffect(ev) and Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0 then
    if duel::negate_effect(d, a.ev as u8) && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,0,LOCATION_GRAVE,1,1,nil,e,tp)
        let sg = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
        // if #sg>0 and Duel.SpecialSummon(sg,0,tp,1-tp,false,false,POS_FACEUP)>0 then
        if !sg.is_empty() && duel::special_summon(d, sg, 0, tp, 1 - tp, false, false, POS_FACEUP as u8, None) > 0 {
            // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsAttackBelow,6000),tp,0,LOCATION_MZONE,nil)
            let g = duel::get_matching_group(
                d,
                Some(&aux::faceup_filter(|d, x| card::is_attack_below(d, x, 6000))),
                tp,
                0,
                LOCATION_MZONE,
                NO_EX,
            );
            // if #g==0 then return end
            if g.is_empty() {
                return;
            }
            // local gg=aux.SelectUnselectGroup(g,e,tp,1,#g,s.rescon,1,tp,HINTMSG_TOGRAVE)
            let rs: aux::Rescon = &rescon;
            let gg = aux::select_unselect_group(
                d,
                &g,
                Some(a.e),
                tp,
                Some(1),
                Some(g.len()),
                Some(rs),
                1,
                tp,
                Some(HINTMSG_TOGRAVE as u64),
                None,
                None,
                false,
            )
            .unwrap_or_default();
            // if #gg>0 then
            if !gg.is_empty() {
                // Duel.HintSelection(gg)
                duel::hint_selection(d, gg.clone(), true);
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoGrave(gg,REASON_EFFECT)
                duel::send_to_grave(d, gg, REASON_EFFECT, None, None);
            }
        }
    }
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `function(e,c,minc) return c==nil or Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0 end`
fn limit_summon_cond(d: &mut Duel, a: Args) -> bool {
    match a.card(1) {
        None => true,
        Some(c) => {
            let p = card::get_controler(d, c);
            duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
        }
    }
}
/// `s.sumfilter(c,handler)`
fn sumfilter(d: &mut Duel, c: CardId, handler: CardId) -> bool {
    // if not c:IsSummonableCard() then return false end
    if !card::is_summonable_card(d, c) {
        return false;
    }
    // local min_req,max_req=c:GetTributeRequirement()
    let (min_req, max_req) = card::get_tribute_requirement(d, c);
    // if min_req>=1 or max_req>=1 then
    if min_req >= 1 || max_req >= 1 {
        // local e1=Effect.CreateEffect(handler)
        let e1 = effect::create_effect(d, handler);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_LIMIT_SUMMON_PROC)
        effect::set_code(d, e1, EFFECT_LIMIT_SUMMON_PROC);
        // e1:SetCondition(function(e,c,minc) return c==nil or Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0 end)
        effect::set_condition(d, e1, Some(lua_fn(limit_summon_cond)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local res=c:IsSummonable(true,nil)
        let res = card::is_summonable(d, c, true, None, 0, None);
        // e1:Reset()
        effect::reset(d, e1);
        // return res
        return res;
    }
    // return false
    false
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SendtoHand(c,nil,REASON_EFFECT)>0 and c:IsLocation(LOCATION_HAND) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None) > 0
        && card::is_location(d, c, LOCATION_HAND)
    {
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // local g=Duel.GetMatchingGroup(s.sumfilter,tp,LOCATION_HAND,0,nil,c)
        let g = duel::get_matching_group(d, Some(&|d, x| sumfilter(d, x, c)), tp, LOCATION_HAND, 0, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
            // local sc=g:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            if let Some(sc) = group::get_first(d, &sel) {
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetDescription(aux.Stringid(id,3))
                effect::set_description(d, e1, aux::stringid(ID, 3));
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_LIMIT_SUMMON_PROC)
                effect::set_code(d, e1, EFFECT_LIMIT_SUMMON_PROC);
                // e1:SetCondition(function(e,c,minc) return c==nil or Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0 end)
                effect::set_condition(d, e1, Some(lua_fn(limit_summon_cond)));
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // sc:RegisterEffect(e1)
                card::register_effect(d, sc, e1, false);
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Summon(tp,sc,true,nil)
                duel::summon(d, tp, sc, true, None, 0, None);
            }
        }
    }
}
