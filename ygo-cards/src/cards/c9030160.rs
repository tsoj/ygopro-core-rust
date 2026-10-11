//! Dark Contract with the Eternal Darkness
//! 常闇の契約書
use super::prelude::*;

pub const ID: u32 = 9030160;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Prevent effect target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetValue(s.evalue)
    effect::set_value_fn(d, e2, lua_fn(evalue));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Prevent tribute
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(EFFECT_UNRELEASABLE_SUM)
    effect::set_code(d, e3, EFFECT_UNRELEASABLE_SUM);
    // e3:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e3, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(condition(cond)));
    // e3:SetValue(s.sumlimit)
    effect::set_value_fn(d, e3, lua_fn(sumlimit));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e5=e3:Clone()
    let e5 = effect::clone(d, e3);
    // e5:SetCode(EFFECT_CANNOT_BE_MATERIAL)
    effect::set_code(d, e5, EFFECT_CANNOT_BE_MATERIAL);
    // e5:SetValue(aux.AND(s.matlimit,aux.cannotmatfilter(SUMMON_TYPE_FUSION,SUMMON_TYPE_SYNCHRO,SUMMON_TYPE_XYZ)))
    // (the combinator is written out: matlimit first, the cannot-material filter only when it is true)
    let cannot = aux::cannotmatfilter(&[SUMMON_TYPE_FUSION, SUMMON_TYPE_SYNCHRO, SUMMON_TYPE_XYZ]);
    let mat = LuaFn::new(move |d, args| {
        if !matlimit_and(d, args) {
            return Value::Bool(false);
        }
        if !cannot.call(d, args).truthy() {
            return Value::Bool(false);
        }
        Value::Bool(true)
    });
    effect::set_value_fn(d, e5, mat);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // --Inflict 2000 damage
    // local e8=Effect.CreateEffect(c)
    let e8 = effect::create_effect(d, c);
    // e8:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e8, aux::stringid(ID, 0));
    // e8:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e8, CATEGORY_DAMAGE);
    // e8:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e8, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e8:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e8, EVENT_PHASE | PHASE_STANDBY);
    // e8:SetRange(LOCATION_SZONE)
    effect::set_range(d, e8, LOCATION_SZONE);
    // e8:SetCountLimit(1)
    effect::set_count_limit(d, e8, 1, 0, 0, 0);
    // e8:SetCondition(s.damcon)
    effect::set_condition(d, e8, Some(condition(damcon)));
    // e8:SetTarget(s.damtg)
    effect::set_target(d, e8, Some(cost_target(damtg)));
    // e8:SetOperation(s.damop)
    effect::set_operation(d, e8, Some(operation(damop)));
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
}
/// `s.matlimit(e,c,sumtype,tp)`
fn matlimit_and(d: &mut Duel, args: &[Value]) -> bool {
    let a = Args(args);
    // if tp==PLAYER_NONE then tp=c:GetControler() end
    let tp = if a.int(3) == PLAYER_NONE as i64 { card::get_controler(d, a.c(1)) } else { a.int(3) as u8 };
    // return e:GetHandlerPlayer()==1-tp
    let hp = effect::get_handler_player(d, a.e(0));
    hp == 1 - tp
}
/// `s.condition(e)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsSetCard,e:GetHandlerPlayer(),LOCATION_PZONE,0,2,nil,SET_DD)
    let hp = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_set_card(d, c, &[SET_DD as u16])), hp, LOCATION_PZONE, 0, 2, NO_EX)
}
/// `s.sumlimit(e,c)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    // if not c then return false end
    let Some(c) = a.card(1) else {
        return false;
    };
    // return not c:IsControler(e:GetHandlerPlayer())
    let hp = effect::get_handler_player(d, a.e(0));
    !card::is_controler(d, c, hp)
}
/// `s.evalue(e,re,rp)`
fn evalue(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // return re:IsSpellTrapEffect() and rp==1-e:GetHandlerPlayer()
    let hp = effect::get_handler_player(d, e);
    effect::is_spell_trap_effect(d, a.e(1)) && a.int(2) == 1 - hp as i64
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(1000)
    duel::set_target_param(d, 1000);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,0,0,tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, tp, 1000);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dmg = info[1].int();
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, p, dmg, REASON_EFFECT, false, None);
}
