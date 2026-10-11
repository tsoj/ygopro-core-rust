//! Propa Gandake
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 61681816;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change race
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.chtg)
    effect::set_target(d, e1, Some(cost_target(chtg)));
    // e1:SetOperation(s.chop)
    effect::set_operation(d, e1, Some(operation(chop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.getraces(g)`
fn getraces(d: &mut Duel, g: &CardSet) -> u64 {
    // local rs=0
    let mut rs: u64 = 0;
    // for c in g:Iter() do
    for c in group::iter(d, g) {
        // rs=rs|c:GetRace()
        rs |= card::get_race(d, c);
    }
    // return rs&(RACE_BEAST|RACE_INSECT|RACE_PLANT|RACE_ROCK)
    rs & (RACE_BEAST | RACE_INSECT | RACE_PLANT | RACE_ROCK)
}
/// `s.chtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn chtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    // local rs=s.getraces(Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,e:GetHandler()))
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, 0, c.into());
    let rs = getraces(d, &g);
    // if chk==0 then return rs>0 end
    if a.chk == Some(0) {
        return rs > 0;
    }
    // e:SetLabel(Duel.AnnounceRace(tp,1,rs))
    let race = duel::announce_race(d, tp, 1, rs);
    effect::set_label(d, a.e, &[race as i64]);
    true
}
/// `s.chop(e,tp,eg,ep,ev,re,r,rp)`
fn chop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local race=e:GetLabel()
    let race = effect::get_label(d, a.e);
    // if race<=0 or not c:IsRelateToEffect(e) then return end
    if race <= 0 || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Change this card's race
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_RACE)
    effect::set_code(d, e1, EFFECT_CHANGE_RACE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(race)
    effect::set_value(d, e1, race);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // if not c:IsImmuneToEffect(e) and c:IsRace(race) then
    if !card::is_immune_to_effect(d, c, a.e) && card::is_race(d, c, race as u64) {
        // Change other monsters' race
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
        effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
        // e2:SetTarget(function(e,c) return c~=e:GetHandler() end)
        effect::set_target(d, e2, Some(effect_card_fn(e2_target)));
        // e2:SetValue(function(e) return e:GetHandler():GetRace() end)
        effect::set_value_fn(d, e2, lua_fn(e2_value));
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
        // Cannot target monsters with the same race as this card
        // local e3=e2:Clone()
        let e3 = effect::clone(d, e2);
        // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
        effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
        // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e3:SetTarget(function(e,c) return c:IsRace(e:GetHandler():GetRace()) end)
        effect::set_target(d, e3, Some(effect_card_fn(e3_target)));
        // e3:SetValue(s.value)
        effect::set_value_fn(d, e3, lua_fn(value));
        // c:RegisterEffect(e3)
        card::register_effect(d, c, e3, false);
    }
}
/// `function(e,c) return c~=e:GetHandler() end` (of `e2`)
fn e2_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    c != h
}
/// `function(e) return e:GetHandler():GetRace() end` (value of `e2`)
fn e2_value(d: &mut Duel, a: Args) -> i64 {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_race(d, h) as i64
}
/// `function(e,c) return c:IsRace(e:GetHandler():GetRace()) end` (target of `e3`)
fn e3_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    let race = card::get_race(d, h);
    card::is_race(d, c, race)
}
/// `s.value(e,re,rp)`
fn value(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    // return re:IsMonsterEffect() and re:GetActivateLocation()==LOCATION_MZONE and re:IsCardControler(1-e:GetHandlerPlayer())
    //     and re:IsCardRace(e:GetHandler():GetRace())
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    if effect::get_activate_location(d, re) != LOCATION_MZONE {
        return false;
    }
    let hp = effect::get_handler_player(d, e);
    if !effect::is_card_controler(d, re, 1 - hp) {
        return false;
    }
    let h = effect::get_handler(d, e).nil();
    let race = card::get_race(d, h);
    effect::is_card_race(d, re, race)
}
