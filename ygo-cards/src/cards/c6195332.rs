//! Exstellarknight Constellar Ptolemy Ω7
use super::prelude::*;

pub const ID: u32 = 6195332;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_TELLARKNIGHT, SET_CONSTELLAR],
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 7 monsters OR 1 "tellarknight" or "Constellar" Xyz Monster you control
    // Xyz.AddProcedure(c,nil,7,3,s.ovfilter,aux.Stringid(id,0),3,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(7),
        3,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(3)),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // While you have 7 or more "tellarknight" monsters with different names in your GY and/or banishment, this card gains 2700 ATK/DEF, also it is unaffected by your opponent's activated effects
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE);
    // e1a:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1a, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1a:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1a, EFFECT_UPDATE_ATTACK);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetCondition(s.atkdefimmcon)
    effect::set_condition(d, e1a, Some(lua_fn(|d, a: Args| atkdefimmcon(d, a.e(0)))));
    // e1a:SetValue(2700)
    effect::set_value(d, e1a, 2700);
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e1b, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // local e1c=e1a:Clone()
    let e1c = effect::clone(d, e1a);
    // e1c:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1c, EFFECT_IMMUNE_EFFECT);
    // e1c:SetValue(function(e,te) return te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsActivated() end)
    effect::set_value_fn(d, e1c, lua_fn(|d, a: Args| {
        let e = a.e(0);
        let te = a.e(1);
        effect::get_owner_player(d, te) != effect::get_handler_player(d, e) && effect::is_activated(d, te)
    }));
    // c:RegisterEffect(e1c)
    card::register_effect(d, c, e1c, false);
    // Shuffle opponent's monsters into the deck based on the number of materials you detach from this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(Cost.DetachFromSelf(1,function(e,tp) return Duel.GetTargetCount(Card.IsAbleToDeck,tp,0,LOCATION_MZONE,nil) end,function(e,og) e:SetLabel(#og) end))
    effect::set_cost(
        d,
        e2,
        Some(cost::detach_from_self(
            cost::Count::N(1),
            Some(cost::Count::F(std::sync::Arc::new(|d: &mut Duel, _e: EffectId, tp: u8| {
                duel::get_target_count(d, Some(&|d, c| card::is_able_to_deck(d, c)), tp, 0, LOCATION_MZONE, NO_EX) as i64
            }))),
            Some(std::sync::Arc::new(|d: &mut Duel, e: EffectId, og: &CardSet| {
                effect::set_label(d, e, &[og.len() as i64]);
            })),
        )),
    );
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.ovfilter(c,tp,xyzc)
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return c:IsSetCard({SET_TELLARKNIGHT,SET_CONSTELLAR},xyzc,SUMMON_TYPE_XYZ,tp) and c:IsType(TYPE_XYZ,xyzc,SUMMON_TYPE_XYZ,tp) and c:IsFaceup()
    card::is_set_card_ctx(d, c, &[SET_TELLARKNIGHT as u16, SET_CONSTELLAR as u16], Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_type_ctx(d, c, TYPE_XYZ, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_faceup(d, c)
}
// function s.xyzop(e,tp,chk)
fn xyzop(d: &mut Duel, a: Args) -> Value {
    let tp = a.p(1);
    // if chk==0 then return Duel.IsMainPhase2(tp) and not Duel.HasFlagEffect(tp,id) end
    if a.int(2) == 0 {
        return Value::Bool(duel::is_main_phase2(d, Some(tp)) && !duel::has_flag_effect(d, tp, ID, None));
    }
    // return Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,EFFECT_FLAG_OATH,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0).map_or(Value::Nil, Value::Effect)
}
// function s.atkdefimmfilter(c)
fn atkdefimmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TELLARKNIGHT) and c:IsMonster() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16]) && card::is_monster(d, c) && card::is_faceup(d, c)
}
// function s.atkdefimmcon(e,c)
fn atkdefimmcon(d: &mut Duel, e: EffectId) -> bool {
    // return Duel.GetMatchingGroup(s.atkdefimmfilter,e:GetHandlerPlayer(),LOCATION_GRAVE|LOCATION_REMOVED,0,nil):GetClassCount(Card.GetCode)>=7
    let p = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&atkdefimmfilter), p, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 7
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsAbleToDeck() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_able_to_deck(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToDeck,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e) as u16;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToDeck,tp,0,LOCATION_MZONE,ct,ct,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_deck(d, c)), a.tp, 0, LOCATION_MZONE, ct, ct, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, a.tp, 0);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
