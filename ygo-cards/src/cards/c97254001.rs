//! Summer
use super::prelude::*;

pub const ID: u32 = 97254001;
/// `local COUNTER_SEASON=0x214`
const COUNTER_SEASON: u16 = 0x214;
/// `local CARD_SPRING=60600821`
const CARD_SPRING: u32 = 60600821;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SPRING], counter_place_list: &[COUNTER_SEASON as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SEASON)
    card::enable_counter_permit(d, c, COUNTER_SEASON, None, None);
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Once per turn: You can place Season Counters on this card equal to the number of your opponent's unused Main Monster Zones
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.countertg)
    effect::set_target(d, e1, Some(cost_target(countertg)));
    // e1:SetOperation(s.counterop)
    effect::set_operation(d, e1, Some(operation(counterop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Once per turn, when your monster declares an attack: You can inflict 400 damage to your opponent for each Season Counter on this card, and for each "Spring" in your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(tp) end)
    effect::set_condition(d, e2, Some(lua_fn(e2_cond)));
    // e2:SetTarget(s.damtg)
    effect::set_target(d, e2, Some(cost_target(damtg)));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Once per turn, during your opponent's End Phase: You can take 1 Field Spell from your hand or Deck that you can place a Season Counter on, and place it face-up on your field (but neither player can activate its effects this turn), and if you do, place all Season Counters on this card on that card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e3, CATEGORY_COUNTER);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e3, Some(lua_fn(e3_cond)));
    // e3:SetTarget(s.pltg)
    effect::set_target(d, e3, Some(cost_target(pltg)));
    // e3:SetOperation(s.plop)
    effect::set_operation(d, e3, Some(operation(plop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,tp) return Duel.GetAttacker():IsControler(tp) end` (e2)
fn e2_cond(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    let at = duel::get_attacker(d).nil();
    card::is_controler(d, at, tp)
}
/// `function(e,tp) return Duel.IsTurnPlayer(1-tp) end` (e3)
fn e3_cond(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    duel::is_turn_player(d, 1 - tp)
}
/// `s.countertg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn countertg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local zones_count=Duel.GetLocationCount(1-tp,LOCATION_MZONE)
    let zones_count = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return zones_count>0 end
    if a.chk == Some(0) {
        return zones_count > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,zones_count,tp,COUNTER_SEASON)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, zones_count as u8, tp, COUNTER_SEASON as i32);
    true
}
/// `s.counterop(e,tp,eg,ep,ev,re,r,rp)`
fn counterop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local zones_count=Duel.GetLocationCount(1-tp,LOCATION_MZONE)
    let zones_count = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0;
    // if c:IsRelateToEffect(e) and zones_count>0 then
    if card::is_relate_to_effect(d, c, a.e) && zones_count > 0 {
        // c:AddCounter(COUNTER_SEASON,zones_count)
        card::add_counter(d, c, COUNTER_SEASON, zones_count as u16, false);
    }
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local season_counter_count=e:GetHandler():GetCounter(COUNTER_SEASON)
    let h = effect::get_handler(d, a.e).nil();
    let season_counter_count = card::get_counter(d, h, COUNTER_SEASON) as i64;
    // local spring_count=Duel.GetMatchingGroupCount(Card.IsCode,tp,LOCATION_GRAVE,0,nil,CARD_SPRING)
    let spring_count = duel::get_matching_group_count(d, Some(&|d, c| card::is_code(d, c, &[CARD_SPRING])), tp, LOCATION_GRAVE, 0, NO_EX) as i64;
    // local total_damage=400*(season_counter_count+spring_count)
    let total_damage = 400 * (season_counter_count + spring_count);
    // if chk==0 then return total_damage>0 end
    if a.chk == Some(0) {
        return total_damage > 0;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,total_damage)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, total_damage as i32);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local player=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let player = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local season_counter_count=c:IsRelateToEffect(e) and c:GetCounter(COUNTER_SEASON) or 0
    let season_counter_count = if card::is_relate_to_effect(d, c, a.e) { card::get_counter(d, c, COUNTER_SEASON) as i64 } else { 0 };
    // local spring_count=Duel.GetMatchingGroupCount(Card.IsCode,tp,LOCATION_GRAVE,0,nil,CARD_SPRING)
    let spring_count = duel::get_matching_group_count(d, Some(&|d, x| card::is_code(d, x, &[CARD_SPRING])), tp, LOCATION_GRAVE, 0, NO_EX) as i64;
    // local total_damage=400*(season_counter_count+spring_count)
    let total_damage = 400 * (season_counter_count + spring_count);
    // if total_damage>0 then
    if total_damage > 0 {
        // Duel.Damage(player,total_damage,REASON_EFFECT)
        duel::damage(d, player, total_damage, REASON_EFFECT, false, None);
    }
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFieldSpell() and c:IsCanAddCounter(COUNTER_SEASON,1,false,LOCATION_ONFIELD) and not c:IsForbidden()
    card::is_field_spell(d, c) && card::is_can_add_counter(d, c, COUNTER_SEASON, 1, false, LOCATION_ONFIELD) && !card::is_forbidden(d, c)
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&plfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,e:GetHandler():GetCounter(COUNTER_SEASON),tp,COUNTER_SEASON)
    let h = effect::get_handler(d, a.e).nil();
    let n = card::get_counter(d, h, COUNTER_SEASON) as u8;
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, n, tp, COUNTER_SEASON as i32);
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil):GetFirst()
    let sg = duel::select_matching_card(d, tp, Some(&plfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sg);
    // if not sc then return end
    let Some(sc) = sc else { return };
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local ct=c:GetCounter(COUNTER_SEASON)
    let ct = card::get_counter(d, c, COUNTER_SEASON);
    // local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
    let fc = duel::get_field_card(d, tp, LOCATION_FZONE, 0);
    // if fc then
    if let Some(fc) = fc {
        // Duel.SendtoGrave(fc,REASON_RULE)
        duel::send_to_grave(d, fc, REASON_RULE, None, None);
        // Duel.BreakEffect()
        duel::break_effect(d);
    }
    // if Duel.MoveToField(sc,tp,tp,LOCATION_FZONE,POS_FACEUP,true) then
    if duel::move_to_field(d, sc, tp, tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None) {
        // Neither player can activate its effects this turn
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3302)
        effect::set_description(d, e1, 3302);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // sc:RegisterEffect(e1)
        card::register_effect(d, sc, e1, false);
        // if ct>0 then
        if ct > 0 {
            // sc:AddCounter(COUNTER_SEASON,ct)
            card::add_counter(d, sc, COUNTER_SEASON, ct as u16, false);
        }
    }
}
