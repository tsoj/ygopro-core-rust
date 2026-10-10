//! Mecha Phantom Beast Auroradon
//! Scripted by ahtelel
use super::prelude::*;

pub const ID: u32 = 44097050;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MECHA_PHANTOM_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_MACHINE),2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_MACHINE, sc, st, Some(tp)))), 2, None, None, None, None);
    // spsummon token
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy/spsummon/salvage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.tg)
    effect::set_target(d, e2, Some(cost_target(tg)));
    // e2:SetOperation(s.op)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `IsPlayerCanSpecialSummonMonster(tp,id+1,SET_MECHA_PHANTOM_BEAST,TYPES_TOKEN,0,0,3,RACE_MACHINE,ATTRIBUTE_WIND)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_MECHA_PHANTOM_BEAST as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(3),
        race: Some(RACE_MACHINE),
        attribute: Some(ATTRIBUTE_WIND),
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //   and Duel.GetLocationCount(tp,LOCATION_MZONE)>2
    //   and Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 2
            && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,3,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 3, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,3,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 3, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and ... then
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 2
        && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0)
    {
        // local token1=Duel.CreateToken(tp,id+1)
        // Duel.SpecialSummonStep(token1,0,tp,tp,false,false,POS_FACEUP)
        let token1 = duel::create_token(d, tp, ID + 1).nil();
        duel::special_summon_step(d, token1, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // local token2=Duel.CreateToken(tp,id+1)
        // Duel.SpecialSummonStep(token2,0,tp,tp,false,false,POS_FACEUP)
        let token2 = duel::create_token(d, tp, ID + 1).nil();
        duel::special_summon_step(d, token2, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // local token3=Duel.CreateToken(tp,id+1)
        // Duel.SpecialSummonStep(token3,0,tp,tp,false,false,POS_FACEUP)
        let token3 = duel::create_token(d, tp, ID + 1).nil();
        duel::special_summon_step(d, token3, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let _ = d;
    // return (sumtype&SUMMON_TYPE_LINK)==SUMMON_TYPE_LINK
    let sumtype = a.int(3);
    (sumtype & SUMMON_TYPE_LINK as i64) == SUMMON_TYPE_LINK as i64
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MECHA_PHANTOM_BEAST) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP)
    card::is_set_card(d, c, &[SET_MECHA_PHANTOM_BEAST as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None)
}
/// `s.svfilter(c,tp)`
fn svfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsTrap() and c:IsAbleToHand()
    card::is_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.tg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=Duel.IsExistingMatchingCard(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) and Duel.CheckReleaseGroupCost(tp,aux.TRUE,1,false,nil,nil)
    let b1 = duel::is_existing_matching_card(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
        && duel::check_release_group_cost(d, tp, None, 1, 1, false, None, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) and Duel.GetLocationCount(tp,LOCATION_MZONE)>=0 and Duel.CheckReleaseGroupCost(tp,aux.TRUE,2,false,nil,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= 0
        && duel::check_release_group_cost(d, tp, None, 2, 2, false, None, NO_EX);
    // local b3=Duel.IsExistingMatchingCard(s.svfilter,tp,LOCATION_GRAVE,0,1,nil) and Duel.CheckReleaseGroupCost(tp,aux.TRUE,3,false,nil,nil)
    let b3 = duel::is_existing_matching_card(d, Some(&svfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::check_release_group_cost(d, tp, None, 3, 3, false, None, NO_EX);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local stab={}
    // local dtab={}
    let mut stab: Vec<i64> = Vec::new();
    let mut dtab: Vec<u64> = Vec::new();
    // if b1 then
    if b1 {
        // table.insert(stab,1)
        stab.push(1);
        // table.insert(dtab,aux.Stringid(id,3))
        dtab.push(aux::stringid(ID, 3));
    }
    // if b2 then
    if b2 {
        stab.push(2);
        dtab.push(aux::stringid(ID, 4));
    }
    // if b3 then
    if b3 {
        stab.push(3);
        dtab.push(aux::stringid(ID, 5));
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EFFECT)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EFFECT as u64);
    // local op=Duel.SelectOption(tp,table.unpack(dtab))+1
    // local sel=stab[op]
    let op = duel::select_option(d, tp, true, &dtab) as usize;
    let sel = stab[op];
    // local g=Duel.SelectReleaseGroupCost(tp,aux.TRUE,sel,sel,false,nil,nil)
    let g = duel::select_release_group_cost(d, tp, None, sel as usize, sel as usize, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g.clone(), REASON_COST, None);
    // e:SetLabel(sel)
    effect::set_label(d, a.e, &[sel]);
    // if sel==1 then
    if sel == 1 {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    // elseif sel==2 then
    } else if sel == 2 {
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 0, tp, 1);
    // elseif sel==3 then
    } else if sel == 3 {
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_GRAVE as i32);
    }
    true
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e);
    // local sel=e:GetLabel()
    let sel = effect::get_label(d, a.e);
    // if sel==1 then
    if sel == 1 {
        // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            // local dg=g:Select(tp,1,1,nil)
            let dg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(dg)
            duel::hint_selection(d, dg.clone(), true);
            // Duel.Destroy(dg,REASON_EFFECT)
            duel::destroy(d, dg, REASON_EFFECT, None, None);
        }
    // elseif sel==2 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    } else if sel == 2 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &g);
        // if tc then
        if let Some(tc) = tc {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    // elseif sel==3 then
    } else if sel == 3 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.svfilter),tp,LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(svfilter)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
