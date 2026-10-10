//! Time Engine
use super::prelude::*;

pub const ID: u32 = 15216188;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_METALMORPH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 monster you controlled that was destroyed by battle or an opponent's card effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e1, EVENT_CUSTOM + ID);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Keep track of the destroyed monsters
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // s.desgroup=Group.CreateGroup()
        set_s_var(d, ID, 0, Value::Group(CardSet::new()));
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_DESTROYED)
        effect::set_code(d, ge1, EVENT_DESTROYED);
        // ge1:SetOperation(s.desgroupregop)
        effect::set_operation(d, ge1, Some(operation(desgroupregop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.desgroup` (the script variable, kept in the duel)
fn desgroup(d: &Duel) -> CardSet {
    match s_var(d, ID, 0) {
        Value::Group(g) => g,
        _ => CardSet::new(),
    }
}
fn desgroupregopfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and c:IsReasonPlayer(1-c:GetPreviousControler())))
    //   and c:IsPreviousLocation(LOCATION_MZONE)
    (card::is_reason(d, c, REASON_BATTLE)
        || (card::is_reason(d, c, REASON_EFFECT) && card::is_reason_player(d, c, 1 - card::get_previous_controler(d, c))))
        && card::is_previous_location(d, c, LOCATION_MZONE)
}
fn desgroupregop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=eg:Filter(s.desgroupregopfilter,nil)
    let eg = a.eg();
    let tg = group::filter(d, &eg, &|d, c| desgroupregopfilter(d, c), NO_EX);
    // if #tg>0 then
    if group::get_count(&tg) > 0 {
        // for tc in tg:Iter() do tc:RegisterFlagEffect(id,RESET_CHAIN,0,1) end
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        let mut dg = desgroup(d);
        // if Duel.GetCurrentChain()==0 then s.desgroup:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            dg.clear();
        }
        // s.desgroup:Merge(tg)
        group::add_card(&mut dg, &tg);
        // s.desgroup:Remove(function(c) return not c:HasFlagEffect(id) end,nil)
        group::remove(d, &mut dg, &|d, c| !card::has_flag_effect(d, c, ID, None), NO_EX);
        set_s_var(d, ID, 0, Value::Group(dg.clone()));
        // Duel.RaiseEvent(s.desgroup,EVENT_CUSTOM+id,e,0,tp,tp,0)
        duel::raise_event(d, dg, EVENT_CUSTOM + ID, Some(a.e), 0, tp, tp, 0);
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsPreviousControler(tp) and c:IsCanBeEffectTarget(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_previous_controler(d, c, tp)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local g=s.desgroup:Filter(s.spfilter,nil,e,tp)
    let dg = desgroup(d);
    let g = group::filter(d, &dg, &|d, c| spfilter(d, c, e, tp), NO_EX);
    // if chkc then return g:IsContains(chkc) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then return #g>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return group::get_count(&g) > 0 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // local tg=nil
    // if #g==1 then
    let tg: CardSet = if group::get_count(&g) == 1 {
        // tg=g
        g.clone()
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // tg=g:Select(tp,1,1,nil)
        group::select(d, &g, tp, 1, 1, false, NO_EX).nil()
    };
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // local dg=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,nil)
    let dg2 = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tg.clone()), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,dg,#dg,tp,0)
    let n = group::get_count(&dg2) as u8;
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, Some(dg2), n, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,tg:GetFirst():GetBaseAttack())
    let first = group::get_first(d, &tg).nil();
    let atk = card::get_base_attack(d, first);
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, atk);
    true
}
fn metalmorphfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_METALMORPH) and c:IsTrap() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_METALMORPH as u16]) && card::is_trap(d, c) && card::is_faceup(d, c)
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local exc=c:IsRelateToEffect(e) and c or nil
    let exc: Exclude = if card::is_relate_to_effect(d, c, e) { c.into() } else { NO_EX };
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)>0
    if card::is_relate_to_effect(d, tc, e)
        && duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        // and tc:IsLevelAbove(5) and tc:IsRace(RACE_MACHINE)
        && card::is_level_above(d, tc, 5)
        && card::is_race(d, tc, RACE_MACHINE)
        // and Duel.IsExistingMatchingCard(s.metalmorphfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,1,exc)
        && duel::is_existing_matching_card(d, Some(&metalmorphfilter), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, exc)
        // and Duel.IsExistingMatchingCard(nil,tp,0,LOCATION_MZONE,1,nil)
        && duel::is_existing_matching_card(d, None, tp, 0, LOCATION_MZONE, 1, NO_EX)
        // and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // local dg=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,nil)
        let dg = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
        // if #dg==0 then return end
        if group::get_count(&dg) == 0 {
            return;
        }
        // local atk=tc:GetBaseAttack()
        let atk = card::get_base_attack(d, tc);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.Destroy(dg,REASON_EFFECT)>0 and tc:IsFaceup() and atk>0
        //   and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if duel::destroy(d, &dg, REASON_EFFECT, None, None) > 0
            && card::is_faceup(d, tc)
            && atk > 0
            && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Damage(1-tp,atk,REASON_EFFECT)
            duel::damage(d, 1 - tp, atk as i64, REASON_EFFECT, false, None);
        }
    }
}
