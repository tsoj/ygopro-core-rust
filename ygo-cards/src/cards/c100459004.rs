//! Blue-Eyes Ultimate Dragon, the God-Surpassing Servant
//! 神を超えたしもべ－青眼の究極竜
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 100459004;

pub const INFO: ScriptInfo = ScriptInfo {
    material_setcode: &[SET_BLUE_EYES],
    listed_names: &[CARD_BLUEEYES_W_DRAGON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: "Blue-Eyes White Dragon" + "Blue-Eyes White Dragon" + "Blue-Eyes White Dragon"
    // Fusion.AddProcMixN(c,true,true,CARD_BLUEEYES_W_DRAGON,3)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::Code(CARD_BLUEEYES_W_DRAGON), 3)]);
    // Cannot be destroyed by card effects, also unaffected by your opponent's activated effects during your opponent's turn
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE);
    // e1a:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1a, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1a:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1a, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetValue(1)
    effect::set_value(d, e1a, 1);
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1b, EFFECT_IMMUNE_EFFECT);
    // e1b:SetCondition(function(e) return Duel.IsTurnPlayer(1-e:GetHandlerPlayer()) end)
    effect::set_condition(d, e1b, Some(condition(|d, a| duel::is_turn_player(d, 1 - effect::get_handler_player(d, a.e)))));
    // e1b:SetValue(function(e,te) return te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsActivated() end)
    effect::set_value_fn(
        d,
        e1b,
        lua_fn(|d, a| {
            let te = a.e(1);
            effect::get_owner_player(d, te) != effect::get_handler_player(d, a.e(0)) && effect::is_activated(d, te)
        }),
    );
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Can make up to 3 attacks during each Battle Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e2, EFFECT_EXTRA_ATTACK);
    // e2:SetValue(2)
    effect::set_value(d, e2, 2);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Once per Chain, during the Battle Phase (Quick Effect): You can target 1 monster that was Special Summoned this turn, or 1 face-down card, that your opponent controls; destroy it. You can only use this effect of "Blue-Eyes Ultimate Dragon, the God-Surpassing Servant" thrice per turn
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(3,id)
    effect::set_count_limit(d, e3, 3, ID, 0, 0);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsBattlePhase() end)
    effect::set_condition(d, e3, Some(condition(|d, _a| duel::is_battle_phase(d, None))));
    // e3:SetCost(Cost.SoftOncePerChain(id))
    effect::set_cost(d, e3, Some(cost::soft_once_per_chain(ID, None)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // e3:SetHintTiming(TIMING_CHAIN_END|TIMING_BATTLE_END,TIMING_CHAIN_END|TIMING_BATTLE_START|TIMING_BATTLE_STEP_END|TIMING_BATTLE_END)
    effect::set_hint_timing(d, e3, TIMING_CHAIN_END | TIMING_BATTLE_END, Some(TIMING_CHAIN_END | TIMING_BATTLE_START | TIMING_BATTLE_STEP_END | TIMING_BATTLE_END));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}

/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsMonster() and c:IsStatus(STATUS_SPSUMMON_TURN)) or c:IsFacedown()
    (card::is_monster(d, c) && card::is_status(d, c, STATUS_SPSUMMON_TURN)) || card::is_facedown(d, c)
}

/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() and s.desfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_on_field(d, chkc) && desfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&desfilter), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&desfilter), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, a.tp, 0);
    true
}

/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::get_first_target(d).first().copied().nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
