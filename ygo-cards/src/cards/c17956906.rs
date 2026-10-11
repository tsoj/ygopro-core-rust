//! Final Light
use super::prelude::*;

pub const ID: u32 = 17956906;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VALKYRIE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // for _,te in ipairs({Duel.GetPlayerEffect(tp,EFFECT_LPCOST_CHANGE)}) do
    for te in duel::get_player_effect(d, a.tp, EFFECT_LPCOST_CHANGE) {
        // local val=te:GetValue()
        let val = effect::get_value(d, te);
        // if val(te,e,tp,1000)~=1000 then return false end
        let Value::Fn(f) = val else {
            crate::lua_error("attempt to call a nil value");
        };
        let r = f.call(d, &[Value::Effect(te), Value::Effect(a.e), Value::Int(a.tp as i64), Value::Int(1000)]);
        if !matches!(r, Value::Int(1000)) {
            return false;
        }
    }
    // return true
    true
}
// function s.filter1(c,e,tp)
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_VALKYRIE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and c:IsCanBeEffectTarget(e)
    card::is_monster(d, c)
        && card::is_set_card(d, c, &[SET_VALKYRIE as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_can_be_effect_target(d, c, Some(e))
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // e:SetLabel(10)
        effect::set_label(d, a.e, &[10]);
        // return true
        return true;
    }
    // (falls off the end: nil, unused)
    true
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_GRAVE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, x| filter1(d, x, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local fg=g:GetClassCount(Card.GetCode)
    let fg = group::get_class_count(d, &g, &|d, x| card::get_code(d, x)[0] as i64);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=10 then return false end
        if effect::get_label(d, a.e) != 10 {
            return false;
        }
        // return ft>0 and #g>0 and Duel.CheckLPCost(tp,1000)
        return ft > 0 && !g.is_empty() && duel::check_lp_cost(d, a.tp, 1000);
    }
    // local n = Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and 1 or math.min(fg,ft)
    let n = if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        1
    } else {
        std::cmp::min(fg as i32, ft)
    };
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    // local pay_list={}
    let mut pay_list: Vec<u64> = Vec::new();
    // for p=1, n do if Duel.CheckLPCost(tp,1000*p) then table.insert(pay_list,p) end end
    for p in 1..=n {
        if duel::check_lp_cost(d, a.tp, (1000 * p) as u32) {
            pay_list.push(p as u64);
        }
    }
    // local pay=Duel.AnnounceNumber(tp,table.unpack(pay_list))
    let pay = duel::announce_number(d, a.tp, &pay_list).nil().0;
    // Duel.PayLPCost(tp,pay*1000)
    duel::pay_lp_cost(d, a.tp, (pay * 1000) as u32);
    // local sg=aux.SelectUnselectGroup(g,e,tp,pay,pay,aux.dncheck,1,tp,HINTMSG_SPSUMMON)
    let rescon = aux::dncheck();
    let sg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(pay as usize),
        Some(pay as usize),
        Some(&rescon),
        1,
        a.tp,
        Some(HINTMSG_SPSUMMON as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,sg,#sg,tp,LOCATION_GRAVE)
    let n_sg = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(sg), n_sg, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,1-tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.filter2(c,e,sp)
fn filter2(d: &mut Duel, c: CardId, e: EffectId, sp: u8) -> bool {
    // return c:IsAttackBelow(2000) and c:IsCanBeSpecialSummoned(e,0,sp,false,false)
    card::is_attack_below(d, c, 2000) && card::is_can_be_special_summoned(d, c, e, 0, sp, false, false, None, None, None)
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local g1,ft1=Duel.GetTargetCards(e),Duel.GetLocationCount(tp,LOCATION_MZONE)
    let g1_opt = duel::get_target_cards(d, a.e);
    let mut ft1 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft1<=0 then return end
    if ft1 <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft1=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft1 = 1;
    }
    // local sg=g1
    let g1 = g1_opt.nil();
    let mut sg = g1.clone();
    // if #g1>ft1 then
    if g1.len() as i32 > ft1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // sg=g1:Select(tp,ft1,ft1,nil)
        sg = group::select(d, &g1, a.tp, ft1 as u16, ft1 as u16, false, NO_EX).nil();
    }
    // local count=Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
    let count = duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    // if count>0 then
    if count > 0 {
        // local g2,ft2=Duel.GetMatchingGroup(s.filter2,tp,0,LOCATION_GRAVE,nil,e,1-tp),math.min(Duel.GetLocationCount(1-tp,LOCATION_MZONE),count)
        let g2 = duel::get_matching_group(d, Some(&|d, x| filter2(d, x, a.e, 1 - a.tp)), a.tp, 0, LOCATION_GRAVE, NO_EX);
        let mut ft2 = std::cmp::min(duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0, count);
        // if #g2>0 and ft2>0 and Duel.SelectYesNo(1-tp,aux.Stringid(id,1)) then
        if !g2.is_empty() && ft2 > 0 && duel::select_yes_no(d, 1 - a.tp, aux::stringid(ID, 1)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.IsPlayerAffectedByEffect(1-tp,CARD_BLUEEYES_SPIRIT) then ft2=1 end
            if duel::is_player_affected_by_effect(d, 1 - a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
                ft2 = 1;
            }
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_SPSUMMON as u64);
            // sg = g2:Select(1-tp,1,ft2,nil)
            let sg2 = group::select(d, &g2, 1 - a.tp, 1, ft2 as u16, false, NO_EX).nil();
            // Duel.SpecialSummon(sg,0,1-tp,1-tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg2, 0, 1 - a.tp, 1 - a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
