//! Swiftwind Panther Warrior
//! 疾風の豹戦士パンサーウォリアー
use super::prelude::*;

pub const ID: u32 = 77482666;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_TIME_WIZARD, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // During the Main Phase (Quick Effect): You can Tribute 1 other monster from your hand or field, then activate 1 of these effects;
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetCost(s.effcost)
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot declare an attack unless a monster(s) was Tributed this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function() return not Duel.HasFlagEffect(0,id) end)
    effect::set_condition(d, e2, Some(condition(|d, _a| !duel::has_flag_effect(d, 0, ID, None))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Keep track of a monster being Tributed
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, move |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD | EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_RELEASE)
        effect::set_code(d, ge1, EVENT_RELEASE);
        // ge1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp)
        //     if not Duel.HasFlagEffect(0,id) and eg:IsExists(function(c) return c:IsPreviousLocation(LOCATION_MZONE) or (not c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsMonsterCard()) end,1,nil) then
        //         Duel.RegisterFlagEffect(0,id,RESET_PHASE|PHASE_END,0,1)
        //     end
        // end)
        effect::set_operation(
            d,
            ge1,
            Some(operation(|d, a| {
                if !duel::has_flag_effect(d, 0, ID, None) {
                    let eg = a.eg.clone().nil();
                    if group::is_exists(d, &eg, &|d, cc| card::is_previous_location(d, cc, LOCATION_MZONE) || (!card::is_previous_location(d, cc, LOCATION_ONFIELD) && card::is_monster_card(d, cc)), 1, NO_EX) {
                        duel::register_flag_effect(d, 0, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
                    }
                }
            })),
        );
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.spcostfilter(c,e,tp)`
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return Duel.GetMZoneCount(tp,c)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,c,e,tp)
    duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, c.into())
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:ListsCode(CARD_DARK_TIME_WIZARD) and not c:IsCode(id) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::lists_code(d, c, &[CARD_DARK_TIME_WIZARD]) && !card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:ListsCode(CARD_DARK_TIME_WIZARD) and c:IsAbleToGrave()
    card::is_spell_trap(d, c) && card::lists_code(d, c, &[CARD_DARK_TIME_WIZARD]) && card::is_able_to_grave(d, c, None)
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local b1=Duel.CheckReleaseGroupCost(tp,s.spcostfilter,1,true,nil,c,e,tp)
    let b1 = duel::check_release_group_cost(d, tp, Some(&|d, cc| spcostfilter(d, cc, e, tp)), 1, 1, true, None, c.into());
    // local b2=Duel.CheckReleaseGroupCost(tp,nil,1,true,nil,c)
    //     and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil)
    let b2 = duel::check_release_group_cost(d, tp, None, 1, 1, true, None, c.into()) && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local filter_func=not b2 and s.spcostfilter or nil
    let filter_func: Option<Filter> = if !b2 { Some(&|d, cc| spcostfilter(d, cc, e, tp)) } else { None };
    // local g=Duel.SelectReleaseGroupCost(tp,filter_func,1,1,true,nil,c,e,tp)
    let g = duel::select_release_group_cost(d, tp, filter_func, 1, 1, true, None, c.into());
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp)
    let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).unwrap_or(0) as i64;
    // e:SetLabel(op)
    effect::set_label(d, e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(CATEGORY_TOGRAVE)
        effect::set_category(d, e, CATEGORY_TOGRAVE);
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    // if op==1 then
    if op == 1 {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, cc| spfilter(d, cc, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    // elseif op==2 then
    } else if op == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoGrave(g,REASON_EFFECT)
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
    }
}
