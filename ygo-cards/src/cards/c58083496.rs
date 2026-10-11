//! Odd-Eyes Override Dragon
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 58083496;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PERFORMAPAL, SET_MAGICIAN, SET_ODD_EYES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ monsters, including a Pendulum Monster
    // Link.AddProcedure(c,nil,2,3,s.matcheck)
    link::add_procedure(d, c, None, 2, Some(3), Some(std::sync::Arc::new(matcheck)), None, None);
    // You can target 1 other card you control; destroy it, and if you do, draw 1 card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your turn, when your opponent activates a card or effect (Quick Effect): You can discard 1 card; negate that effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsTurnPlayer(tp) and rp==1-tp and Duel.IsChainDisablable(ev) end)
    effect::set_condition(d, e2, Some(lua_fn(e2_cond)));
    // e2:SetCost(Cost.Discard())
    effect::set_cost(d, e2, Some(cost::discard(None, false, None, None, None)));
    // e2:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e2, Some(cost_target(e2_target)));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) Duel.NegateEffect(ev) end)
    effect::set_operation(d, e2, Some(operation(e2_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If you destroyed a Pendulum Monster Card by this card's first effect, during your Main Phase this turn, you can conduct 1 Pendulum Summon of a "Performapal" monster(s), "Magician" Pendulum Monster(s), and/or "Odd-Eyes" monster(s), in addition to your Pendulum Summon (you can only gain this effect once per turn)
    // local extra_pendulum_effect=Pendulum.CreateAdditionalPendulumSummonEffect(c,s.pendulumfilter,nil,aux.Stringid(id,2),id)
    let extra_pendulum_effect = pendulum::create_additional_pendulum_summon_effect(
        d,
        c,
        Some(std::sync::Arc::new(|d: &mut Duel, x: CardId, _e: EffectId, _tp: u8, _l: u32, _r: u32| pendulumfilter(d, x))),
        None,
        Some(aux::stringid(ID, 2)),
        ID,
        None,
        None,
    );
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_GRANT)
    effect::set_type(d, e3a, EFFECT_TYPE_FIELD + EFFECT_TYPE_GRANT);
    // e3a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3a, LOCATION_MZONE);
    // e3a:SetTargetRange(LOCATION_SZONE,0)
    effect::set_target_range(d, e3a, LOCATION_SZONE, 0);
    // e3a:SetTarget(function(e,c) return c:IsLocation(LOCATION_PZONE) end)
    effect::set_target(d, e3a, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_PZONE))));
    // e3a:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(d, e3a, Some(lua_fn(e3a_cond)));
    // e3a:SetLabelObject(extra_pendulum_effect)
    effect::set_label_object(d, e3a, Value::Effect(extra_pendulum_effect));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local harmonic_effect=Pendulum.CreateHarmonicOscillationEffect(c,s.pendulumfilter,aux.Stringid(id,2),id)
    let (harmonic_effect, _) = pendulum::create_harmonic_oscillation_effect(
        d,
        c,
        Some(std::sync::Arc::new(|d: &mut Duel, x: CardId, _e: EffectId, _tp: u8, _l: u32, _r: u32| pendulumfilter(d, x))),
        Some(aux::stringid(ID, 2)),
        ID,
        None,
    );
    // local e3b=e3a:Clone()
    let e3b = effect::clone(d, e3a);
    // e3b:SetLabelObject(harmonic_effect)
    effect::set_label_object(d, e3b, Value::Effect(harmonic_effect));
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
}
/// `s.matcheck(g,lc,sumtype,tp)`
fn matcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsType,1,nil,TYPE_PENDULUM,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_type_ctx(d, c, TYPE_PENDULUM, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsOnField() and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_on_field(d, chkc) && chkc != c;
    }
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1)
    //     and Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1) && duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,0,1,1,c)
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if c:IsRelateToEffect(e) and c:IsFaceup() and tc:IsOriginalType(TYPE_PENDULUM) then
        if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::is_original_type(d, tc, TYPE_PENDULUM) {
            // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,3))
            card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 3));
        }
    }
}
/// `s.pendulumfilter(c)`
fn pendulumfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_PERFORMAPAL,SET_ODD_EYES}) or (c:IsSetCard(SET_MAGICIAN) and c:IsPendulumMonster())
    card::is_set_card(d, c, &[SET_PERFORMAPAL as u16, SET_ODD_EYES as u16])
        || (card::is_set_card(d, c, &[SET_MAGICIAN as u16]) && card::is_pendulum_monster(d, c))
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` condition of e2
fn e2_cond(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    let rp = a.p(7);
    // return Duel.IsTurnPlayer(tp) and rp==1-tp and Duel.IsChainDisablable(ev)
    duel::is_turn_player(d, tp) && rp == 1 - tp && duel::is_chain_disablable(d, a.int(4) as u8)
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)` target of e2
fn e2_target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` operation of e2
fn e2_op(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
/// `function(e) return e:GetHandler():HasFlagEffect(id) end` of e3a
fn e3a_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::has_flag_effect(d, h, ID, None)
}
