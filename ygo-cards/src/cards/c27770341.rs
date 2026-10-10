//! Super Rejuvenation
use super::prelude::*;

pub const ID: u32 = 27770341;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, TIMING_END_PHASE, None);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, move |d| {
        // s[0]=0
        // s[1]=0
        set_s_var(d, ID, 0, Value::Int(0));
        set_s_var(d, ID, 1, Value::Int(0));
        // aux.AddValuesReset(function() s[0]=0 s[1]=0 end)
        aux::add_values_reset(d, reset_counts);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
        effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
        // e3:SetCode(EVENT_RELEASE)
        effect::set_code(d, e3, EVENT_RELEASE);
        // e3:SetOperation(s.addcount)
        effect::set_operation(d, e3, Some(operation(addcount)));
        // Duel.RegisterEffect(e3,0)
        duel::register_effect(d, e3, 0);
        // local e4=Effect.CreateEffect(c)
        let e4 = effect::create_effect(d, c);
        // e4:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
        effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
        // e4:SetCode(EVENT_DISCARD)
        effect::set_code(d, e4, EVENT_DISCARD);
        // e4:SetOperation(s.addcount)
        effect::set_operation(d, e4, Some(operation(addcount)));
        // Duel.RegisterEffect(e4,0)
        duel::register_effect(d, e4, 0);
    });
}
/// `aux.AddValuesReset(function() s[0]=0 s[1]=0 end)`
fn reset_counts(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Int(0));
    set_s_var(d, ID, 1, Value::Int(0));
}
/// `s.addcount(e,tp,eg,ep,ev,re,r,rp)`
fn addcount(d: &mut Duel, a: &EffArgs) {
    // for tc in aux.Next(eg) do
    for tc in group::iter(d, &a.eg()) {
        // local pl=tc:GetPreviousLocation()
        let pl = card::get_previous_location(d, tc);
        // if pl==LOCATION_MZONE and tc:GetPreviousRaceOnField()==RACE_DRAGON then
        if pl == LOCATION_MZONE && card::get_previous_race_on_field(d, tc) == RACE_DRAGON {
            // local p=tc:GetReasonPlayer()
            let p = card::get_reason_player(d, tc);
            // s[p]=s[p]+1
            let v = s_var(d, ID, p as u64).int();
            set_s_var(d, ID, p as u64, Value::Int(v + 1));
        // elseif pl==LOCATION_HAND and tc:IsMonster() and tc:GetOriginalRace()==RACE_DRAGON then
        } else if pl == LOCATION_HAND && card::is_monster(d, tc) && card::get_original_race(d, tc) == RACE_DRAGON {
            // local p=tc:GetPreviousControler()
            let p = card::get_previous_controler(d, tc);
            // s[p]=s[p]+1
            let v = s_var(d, ID, p as u64).int();
            set_s_var(d, ID, p as u64, Value::Int(v + 1));
        }
    }
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetOperation(s.droperation)
    effect::set_operation(d, e1, Some(operation(droperation)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.droperation(e,tp,eg,ep,ev,re,r,rp)`
fn droperation(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Draw(tp,s[tp],REASON_EFFECT)
    let n = s_var(d, ID, a.tp as u64).int() as u32;
    duel::draw(d, a.tp, n, REASON_EFFECT);
}
