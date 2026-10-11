//! Clown Crew Matinee Operatics
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 57847269;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // During your Main Phase: You can Special Summon 1 "Clown Crew" monster from your Deck or Extra Deck, ignoring its Summoning conditions, also you cannot activate the effects of monsters Special Summoned from the Deck or Extra Deck until the end of the next turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During the End Phase: You can draw cards equal to the number of monster card types (Ritual, Fusion, Synchro, Xyz, Pendulum, Link) Tributed this turn, then you can Set 1 Spell/Trap from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DRAW+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_DRAW + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Keep track of how many monster card types (Ritual, Fusion, Synchro, Xyz, Pendulum, Link) were Tributed each turn
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_RELEASE)
        effect::set_code(d, ge1, EVENT_RELEASE);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    // end)
    });
}
// s.listed_series={SET_CLOWN_CREW}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CLOWN_CREW], ..ScriptInfo::DEFAULT };

/// `s.popcount(value)`
fn popcount(mut value: i64) -> i64 {
    // local count = 0
    let mut count = 0;
    // while value > 0 do
    while value > 0 {
        // value = value & (value - 1)
        value &= value - 1;
        // count = count + 1
        count += 1;
    }
    // return count
    count
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local g=eg:Filter(Card.IsType,nil,TYPE_RITUAL|TYPE_PENDULUM|TYPE_EXTRA)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| card::is_type(d, c, TYPE_RITUAL | TYPE_PENDULUM | TYPE_EXTRA), NO_EX);
    // for rc in g:Iter() do
    for rc in group::iter(d, &g) {
        // local card_type=(rc:GetOriginalType()&(TYPE_RITUAL|TYPE_PENDULUM|TYPE_EXTRA))
        let card_type = card::get_original_type(d, rc) & (TYPE_RITUAL | TYPE_PENDULUM | TYPE_EXTRA);
        // local current_label=Duel.GetFlagEffectLabel(0,id) or 0
        let current_label = duel::get_flag_effect_label(d, 0, ID).first().copied().unwrap_or(0);
        // local new_label=current_label|card_type
        let new_label = current_label | card_type as i64;
        // if new_label~=current_label then
        if new_label != current_label {
            // Duel.RegisterFlagEffect(0,id,RESET_PHASE|PHASE_END,0,1)
            duel::register_flag_effect(d, 0, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
            // Duel.SetFlagEffectLabel(0,id,new_label)
            duel::set_flag_effect_label(d, 0, ID, new_label as i32);
        }
    }
}
/// `s.spfilter(c,e,tp,mmz_chk)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    // if not (c:IsSetCard(SET_CLOWN_CREW) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,true,false)) then return false end
    if !(card::is_set_card(d, c, &[SET_CLOWN_CREW as u16])
        && card::is_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None))
    {
        return false;
    }
    // if c:IsLocation(LOCATION_DECK) then
    if card::is_location(d, c, LOCATION_DECK) {
        // return mmz_chk
        mmz_chk
    // elseif c:IsLocation(LOCATION_EXTRA) then
    } else if card::is_location(d, c, LOCATION_EXTRA) {
        // return Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
        duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
    } else {
        // (the Lua function falls through and returns nil)
        false
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE)>0) end
    if a.chk == Some(0) {
        let mmz = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, mmz)), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE)>0)
    let mmz = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, mmz)), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
    }
    // You cannot activate the effects of monsters Special Summoned from the Deck or Extra Deck until the end of the next turn
    // local e1=Effect.CreateEffect(e:GetHandler())
    let owner = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, owner);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(s.aclimit)
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    // e1:SetReset(RESET_PHASE|PHASE_END,2)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let rc = effect::get_handler(d, re).nil();
    // return re:IsMonsterEffect() and rc:IsOnField() and rc:IsSummonLocation(LOCATION_DECK|LOCATION_EXTRA)
    effect::is_monster_effect(d, re) && card::is_on_field(d, rc) && card::is_summon_location(d, rc, LOCATION_DECK | LOCATION_EXTRA)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local types=Duel.GetFlagEffectLabel(0,id) or 0
    let types = duel::get_flag_effect_label(d, 0, ID).first().copied().unwrap_or(0);
    // local ct = s.popcount(types)
    let ct = popcount(types);
    // if chk==0 then return ct>0 and Duel.IsPlayerCanDraw(tp,ct) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_player_can_draw(d, a.tp, ct as u32);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, ct as i32);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // local types=Duel.GetFlagEffectLabel(0,id) or 0
    let types = duel::get_flag_effect_label(d, 0, ID).first().copied().unwrap_or(0);
    // local ct = s.popcount(types)
    let ct = popcount(types);
    // if Duel.Draw(tp,ct,REASON_EFFECT)>0 then
    if duel::draw(d, a.tp, ct as u32, REASON_EFFECT) > 0 {
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
        // if Duel.IsExistingMatchingCard(Card.IsSSetable,tp,LOCATION_HAND,0,1,nil)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
        if duel::is_existing_matching_card(d, Some(&|d, c| card::is_s_setable(d, c, false, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
            // local g=Duel.SelectMatchingCard(tp,Card.IsSSetable,tp,LOCATION_HAND,0,1,1,nil)
            let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_s_setable(d, c, false, None)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SSet(tp,g,tp,false)
                duel::s_set(d, a.tp, g, Some(a.tp), false);
            }
        }
    }
}
