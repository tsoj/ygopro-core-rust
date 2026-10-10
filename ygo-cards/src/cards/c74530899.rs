//! Metaion, the Timelord
use super::prelude::*;

pub const ID: u32 = 74530899;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetRange(LOCATION_DECK)
    effect::set_range(d, e1, LOCATION_DECK);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SUMMON_PROC);
    // e2:SetCondition(s.ntcon)
    effect::set_condition(d, e2, Some(lua_fn(ntcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // indes
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e3:Clone()
    let e5 = effect::clone(d, e3);
    // e5:SetCondition(s.damcon)
    effect::set_condition(d, e5, Some(lua_fn(damcon)));
    // e5:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e5, EFFECT_AVOID_BATTLE_DAMAGE);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // to hand
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e6, aux::stringid(ID, 1));
    // e6:SetCategory(CATEGORY_TOHAND+CATEGORY_DAMAGE)
    effect::set_category(d, e6, CATEGORY_TOHAND + CATEGORY_DAMAGE);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e6:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e6, EVENT_PHASE | PHASE_BATTLE);
    // e6:SetCountLimit(1)
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetCondition(s.thcon)
    effect::set_condition(d, e6, Some(condition(thcon)));
    // e6:SetTarget(s.thtg)
    effect::set_target(d, e6, Some(cost_target(thtg)));
    // e6:SetOperation(s.thop)
    effect::set_operation(d, e6, Some(operation(thop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // to deck
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e7, aux::stringid(ID, 2));
    // e7:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e7, CATEGORY_TODECK);
    // e7:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e7, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e7:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e7, EVENT_PHASE | PHASE_STANDBY);
    // e7:SetCountLimit(1)
    effect::set_count_limit(d, e7, 1, 0, 0, 0);
    // e7:SetRange(LOCATION_MZONE)
    effect::set_range(d, e7, LOCATION_MZONE);
    // e7:SetCondition(s.tdcon)
    effect::set_condition(d, e7, Some(condition(tdcon)));
    // e7:SetTarget(s.tdtg)
    effect::set_target(d, e7, Some(cost_target(tdtg)));
    // e7:SetOperation(s.tdop)
    effect::set_operation(d, e7, Some(operation(tdop)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
/// `s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // return minc==0 and c:GetLevel()>4
    //     and Duel.GetFieldGroupCount(c:GetControler(),LOCATION_MZONE,0)==0
    //     and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    let minc = a.int(2);
    minc == 0 && card::get_level(d, c) > 4 && {
        let p = card::get_controler(d, c);
        duel::get_field_group_count(d, p, LOCATION_MZONE, 0) == 0 && duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
    }
}
/// `s.damcon(e)`
fn damcon(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsAttackPos()
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_attack_pos(d, h)
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetBattledGroupCount()>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_battled_group_count(d, h) > 0
}
/// The group of `Duel.GetMatchingGroup(Card.IsAbleToHand,tp,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())`.
fn to_hand_group(d: &mut Duel, a: &EffArgs) -> CardSet {
    let h = effect::get_handler(d, a.e).nil();
    duel::get_matching_group(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_MZONE, LOCATION_MZONE, h.into())
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())
    let g = to_hand_group(d, a);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g.clone()), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,#g*300)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (g.len() * 300) as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())
    let g = to_hand_group(d, a);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.SendtoHand(g,nil,REASON_EFFECT)
    duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // local ct=og:FilterCount(Card.IsLocation,nil,LOCATION_HAND|LOCATION_EXTRA)
    let ct = group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_HAND | LOCATION_EXTRA), NO_EX);
    // Duel.Damage(1-tp,ct*300,REASON_EFFECT)
    duel::damage(d, 1 - tp, (ct * 300) as i64, REASON_EFFECT, false, None);
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, c, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
