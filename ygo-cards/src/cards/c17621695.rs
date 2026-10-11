//! 糾罪都市－エニアポリス
//! Enneapolis
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 17621695;
pub const INFO: ScriptInfo = ScriptInfo { counter_list: &[COUNTER_EC], listed_series: &[SET_ENNEACRAFT], ..ScriptInfo::DEFAULT };

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
    // Remove all E.C. Counters from your fiend, and if you do, inflict 900 damage to your opponent for each
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return any number of "Enneacraft" Pendulum Monster Cards you control to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.rthtg)
    effect::set_target(d, e2, Some(cost_target(rthtg)));
    // e2:SetOperation(s.rthop)
    effect::set_operation(d, e2, Some(operation(rthop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Return to the hand, or place in your Pendulum Zone, 1 of your "Enneacraft" Pendulum Monsters that was flipped face-up during the Main Phase
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e3, EVENT_CUSTOM + ID);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.thpztg)
    effect::set_target(d, e3, Some(cost_target(thpztg)));
    // e3:SetOperation(s.thpzop)
    effect::set_operation(d, e3, Some(operation(thpzop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e3:SetLabelObject(g)
    effect::set_label_object(d, e3, Value::Group(CardSet::new()));
    // Keep track of "Enneacraft" Pendulum Monsters that were flipped face-up
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e4:SetCode(EVENT_FLIP)
    effect::set_code(d, e4, EVENT_FLIP);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetLabelObject(e3)
    effect::set_label_object(d, e4, Value::Effect(e3));
    // e4:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e4, Some(lua_fn(main_phase_cond)));
    // e4:SetOperation(s.regop)
    effect::set_operation(d, e4, Some(operation(regop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function() return Duel.IsMainPhase() end
fn main_phase_cond(d: &mut Duel, _a: Args) -> bool {
    duel::is_main_phase(d, None)
}
// s.listed_series / s.counter_list are declared in INFO
// function s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ct=Duel.GetCounter(tp,1,0,COUNTER_EC)
    let ct = duel::get_counter(d, tp, 1, 0, COUNTER_EC as u16);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,ct*900)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, (ct * 900) as i32);
    true
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.HasCounter,tp,LOCATION_ONFIELD,0,nil,COUNTER_EC)
    let g = duel::get_matching_group(d, Some(&|d, c| card::has_counter(d, c, COUNTER_EC as u16)), tp, LOCATION_ONFIELD, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local total_count=0
    let mut total_count: i64 = 0;
    // for sc in g:Iter() do
    for sc in group::iter(d, &g) {
        // local sc_count=sc:GetCounter(COUNTER_EC)
        let sc_count = card::get_counter(d, sc, COUNTER_EC as u16);
        // if sc:RemoveCounter(tp,COUNTER_EC,sc_count,REASON_EFFECT) then
        if card::remove_counter(d, sc, tp, COUNTER_EC as u16, sc_count as u16, REASON_EFFECT) {
            // total_count=total_count+sc_count
            total_count += sc_count as i64;
        }
    }
    // if total_count>0 then
    if total_count > 0 {
        // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
        let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
        // Duel.Damage(p,total_count*900,REASON_EFFECT)
        duel::damage(d, p, total_count * 900, REASON_EFFECT, false, None);
    }
}
/// `s.rthfilter(c)`
fn rthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ENNEACRAFT) and c:IsOriginalType(TYPE_PENDULUM) and c:IsFaceup() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ENNEACRAFT as u16])
        && card::is_original_type(d, c, TYPE_PENDULUM)
        && card::is_faceup(d, c)
        && card::is_able_to_hand(d, c, None)
}
// function s.rthtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) and s.rthfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, tp) && rthfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rthfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rthfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.rthfilter,tp,LOCATION_ONFIELD,0,1,99,nil)
    let g = duel::select_target(d, tp, Some(&rthfilter), tp, LOCATION_ONFIELD, 0, 1, 99, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,tp,0)
    let n = g.len();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n as u8, tp, 0);
    true
}
// function s.rthop(e,tp,eg,ep,ev,re,r,rp)
fn rthop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoHand(tg,nil,REASON_EFFECT)
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
    }
}
/// `s.thpzfilter(c,tp)`
fn thpzfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ENNEACRAFT) and c:IsType(TYPE_PENDULUM) and c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    //     and (c:IsAbleToHand() or (not c:IsForbidden() and Duel.CheckPendulumZones(tp)))
    card::is_set_card(d, c, &[SET_ENNEACRAFT as u16])
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && (card::is_able_to_hand(d, c, None) || (!card::is_forbidden(d, c) && duel::check_pendulum_zones(d, tp)))
}
// function s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=eg:Filter(s.thpzfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| thpzfilter(d, c, tp), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do
        for tc in group::iter(d, &tg) {
            // tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e3 = effect::get_label_object(d, a.e).as_effect().nil();
        let mut g = effect::get_label_object(d, e3).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e3, Value::Group(g));
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,tp,0)
        let h = effect::get_handler(d, a.e).nil();
        duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(a.e), 0, tp, tp, 0);
    }
}
// function s.thpztg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thpztg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=e:GetLabelObject():Filter(s.thpzfilter,nil,tp)
    let e_label = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let g = group::filter(d, &e_label, &|d, c| thpzfilter(d, c, tp), NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, tp, 0);
    true
}
// function s.thpzop(e,tp,eg,ep,ev,re,r,rp)
fn thpzop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e):Filter(s.thpzfilter,nil,tp)
    let targets = duel::get_target_cards(d, a.e).nil();
    let g = group::filter(d, &targets, &|d, c| thpzfilter(d, c, tp), NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local sc=nil
    // if #g==1 then
    let sc = if g.len() == 1 {
        // sc=g:GetFirst()
        group::get_first(d, &g)
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
        // sc=g:Select(tp,1,1,nil,tp):GetFirst()
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sel)
    };
    // if not sc then return end
    let Some(sc) = sc else {
        return;
    };
    // Duel.HintSelection(sc)
    duel::hint_selection(d, sc, true);
    // local b1=sc:IsAbleToHand()
    let b1 = card::is_able_to_hand(d, sc, None);
    // local b2=Duel.CheckPendulumZones(tp) and not sc:IsForbidden()
    let b2 = duel::check_pendulum_zones(d, tp) && !card::is_forbidden(d, sc);
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,4)},
    //     {b2,aux.Stringid(id,5)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 4)), (b2, aux::stringid(ID, 5))]);
    // if op==1 then
    if op == Some(1) {
        // Duel.SendtoHand(sc,nil,REASON_EFFECT)
        duel::send_to_hand(d, sc, None, REASON_EFFECT, None);
    } else if op == Some(2) {
        // Duel.MoveToField(sc,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, sc, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
