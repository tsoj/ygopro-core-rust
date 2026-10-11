//! Clown Crew Cappello
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 19491080;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // During your Main Phase: You can send 1 "Clown Crew" card from your Deck to your GY, except a Pendulum Monster, and if you do, Special Summon this card, also you cannot activate the effects of monsters Special Summoned from the Deck or Extra Deck until the end of the next turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetTarget(s.gytg)
    effect::set_target(d, e1, Some(cost_target(gytg)));
    // e1:SetOperation(s.gyop)
    effect::set_operation(d, e1, Some(operation(gyop)));
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Monsters your opponent controls lose 1500 DEF while you control a Tribute Summoned monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetCondition(function(e) return Duel.IsExistingMatchingCard(Card.IsTributeSummoned,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil) end)
    effect::set_condition(d, e2, Some(lua_fn(|d, a: Args| {
        let p = effect::get_handler_player(d, a.e(0));
        duel::is_existing_matching_card(d, Some(&|d, c| card::is_tribute_summoned(d, c)), p, LOCATION_MZONE, 0, 1, NO_EX)
    })));
    // e2:SetValue(-1500)
    effect::set_value(d, e2, -1500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this card is Tributed: You can activate 1 of these effects (but you can only use each of these effects of "Clown Crew Hat" once per turn);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_RELEASE)
    effect::set_code(d, e3, EVENT_RELEASE);
    // e3:SetTarget(s.efftg)
    effect::set_target(d, e3, Some(cost_target(efftg)));
    // e3:SetOperation(s.effop)
    effect::set_operation(d, e3, Some(operation(effop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.listed_series={SET_CLOWN_CREW}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CLOWN_CREW], ..ScriptInfo::DEFAULT };

/// `s.gyfilter(c)`
fn gyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CLOWN_CREW) and not c:IsPendulumMonster() and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_CLOWN_CREW as u16]) && !card::is_pendulum_monster(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.gytg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.gyfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&gyfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.gyop(e,tp,eg,ep,ev,re,r,rp)`
fn gyop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.gyfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&gyfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoGrave(g,REASON_EFFECT)>0 and c:IsRelateToEffect(e) then
    if !g.is_empty() && duel::send_to_grave(d, g, REASON_EFFECT, None, None) > 0 && card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
    // --You cannot activate the effects of monsters Special Summoned from the Deck or Extra Deck until the end of the next turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
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
    // e1:SetValue(s.actval)
    effect::set_value_fn(d, e1, lua_fn(actval));
    // e1:SetReset(RESET_PHASE|PHASE_END,2)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.actval(e,re,tp)`
fn actval(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let rc = effect::get_handler(d, re).nil();
    // return re:IsMonsterEffect() and rc:IsLocation(LOCATION_MZONE) and rc:IsSummonLocation(LOCATION_DECK|LOCATION_EXTRA)
    effect::is_monster_effect(d, re) && card::is_location(d, rc, LOCATION_MZONE) && card::is_summon_location(d, rc, LOCATION_DECK | LOCATION_EXTRA)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if c:IsLocation(LOCATION_DECK) and Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return false end
    if card::is_location(d, c, LOCATION_DECK) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return false;
    }
    // if c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,nil,c)<=0 then return false end
    if card::is_location(d, c, LOCATION_EXTRA) && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 <= 0 {
        return false;
    }
    // return c:IsSetCard(SET_CLOWN_CREW) and c:IsMonster() and not c:IsPendulumMonster() and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_set_card(d, c, &[SET_CLOWN_CREW as u16])
        && card::is_monster(d, c)
        && !card::is_pendulum_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local b1=not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsExistingMatchingCard(aux.AND(Card.IsPendulumMonster,Card.IsFaceup,Card.IsAbleToDeck),tp,LOCATION_MZONE|LOCATION_EXTRA,LOCATION_MZONE|LOCATION_EXTRA,1,nil)
    let b1 = !duel::has_flag_effect(d, a.tp, ID, None)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_pendulum_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)),
            a.tp,
            LOCATION_MZONE | LOCATION_EXTRA,
            LOCATION_MZONE | LOCATION_EXTRA,
            1,
            NO_EX,
        );
    // local b2=not Duel.HasFlagEffect(tp,id+1)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e,tp)
    let b2 = !duel::has_flag_effect(d, a.tp, ID + 1, None)
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,3)},
    //     {b2,aux.Stringid(id,4)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |x| x as i64)]);
    // if op==1 then
    if op == Some(1) {
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_TODECK)
        effect::set_category(d, a.e, CATEGORY_TODECK);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,PLAYER_ALL,LOCATION_MZONE|LOCATION_EXTRA)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, PLAYER_ALL as u8, (LOCATION_MZONE | LOCATION_EXTRA) as i32);
    // elseif op==2 then
    } else if op == Some(2) {
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // --● Shuffle all Pendulum Monsters on the field and face-up Extra Decks into the Deck
        // local g=Duel.GetMatchingGroup(aux.AND(Card.IsPendulumMonster,Card.IsFaceup,Card.IsAbleToDeck),tp,LOCATION_MZONE|LOCATION_EXTRA,LOCATION_MZONE|LOCATION_EXTRA,nil)
        let g = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_pendulum_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)),
            a.tp,
            LOCATION_MZONE | LOCATION_EXTRA,
            LOCATION_MZONE | LOCATION_EXTRA,
            NO_EX,
        );
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    // elseif op==2 then
    } else if op == 2 {
        // --● Special Summon 1 non-Pendulum "Clown Crew" monster from your Deck or Extra Deck, ignoring its Summoning conditions
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,true,false,POS_FACEUP)
            duel::special_summon(d, g, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
        }
    }
}
