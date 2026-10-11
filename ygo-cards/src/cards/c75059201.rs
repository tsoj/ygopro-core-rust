//! F.A. Turbo Charger
use super::prelude::*;

pub const ID: u32 = 75059201;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --increase ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --prevent battle targets
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetTarget(s.atglimit)
    effect::set_target(d, e2, Some(effect_card_fn(atglimit)));
    // e2:SetValue(s.atlimit)
    effect::set_value_fn(d, e2, effect_card_fn(atlimit));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --prevent other targets
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetProperty(EFFECT_FLAG_IGNORE_RANGE+EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(s.tglimit)
    effect::set_target(d, e3, Some(effect_card_fn(tglimit)));
    // e3:SetValue(s.tgval)
    effect::set_value_fn(d, e3, lua_fn(|d, a: Args| tgval(d, a)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --increase level
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e5, CATEGORY_LVCHANGE);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e5:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e5, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e5:SetCode(EVENT_CHAINING)
    effect::set_code(d, e5, EVENT_CHAINING);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCondition(s.lvcon)
    effect::set_condition(d, e5, Some(condition(lvcon)));
    // e5:SetOperation(s.lvop)
    effect::set_operation(d, e5, Some(operation(lvop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // --limits activations
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    // e6:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e6, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e6:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e6, EFFECT_CANNOT_ACTIVATE);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetTargetRange(0,1)
    effect::set_target_range(d, e6, 0, 1);
    // e6:SetValue(s.aclimit)
    effect::set_value_fn(d, e6, lua_fn(|d, a: Args| aclimit(d, a)));
    // e6:SetCondition(s.actcon)
    effect::set_condition(d, e6, Some(condition(actcon)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetLevel()*300
    card::get_level(d, c) as i64 * 300
}
/// `s.atglimit(e,c)`
fn atglimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local lv=e:GetHandler():GetLevel()
    let h = effect::get_handler(d, e).nil();
    let lv = card::get_level(d, h) as i64;
    // if c:GetRank()>0 then
    if card::get_rank(d, c) > 0 {
        // return c:GetOriginalRank()<lv
        (card::get_original_rank(d, c) as i64) < lv
    } else if card::has_level(d, c) {
        // return c:GetOriginalLevel()<lv
        (card::get_original_level(d, c) as i64) < lv
    } else {
        // else return false end
        false
    }
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler()
    let h = effect::get_handler(d, e).nil();
    c != h
}
/// `s.tglimit(e,c)`
fn tglimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler()
    let h = effect::get_handler(d, e).nil();
    c != h
}
/// `s.tgval(e,re,rp)`
fn tgval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    let rp = a.p(2);
    // if not aux.tgoval(e,re,rp) or not re:IsMonsterEffect() then return false end
    if !aux::tgoval().call(d, &[Value::Effect(e), Value::Effect(re), Value::Int(rp as i64)]).truthy() {
        return false;
    }
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    // local c=re:GetHandler()
    let c = effect::get_handler(d, re).nil();
    // local lv=e:GetHandler():GetLevel()
    let h = effect::get_handler(d, e).nil();
    let lv = card::get_level(d, h) as i64;
    // if c:GetRank()>0 then
    if card::get_rank(d, c) > 0 {
        // return c:GetOriginalRank()<lv
        (card::get_original_rank(d, c) as i64) < lv
    } else if card::has_level(d, c) {
        // return c:GetOriginalLevel()<lv
        (card::get_original_level(d, c) as i64) < lv
    } else {
        // else return false end
        false
    }
}
/// `s.lvcon(e,tp,eg,ep,ev,re,r,rp)`
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsSpellTrapEffect() and re:GetHandler():IsSetCard(SET_FA)
    let re = a.re.nil();
    if !effect::is_spell_trap_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_set_card(d, h, &[SET_FA as u16])
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_LEVEL)
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsMonsterEffect()
    effect::is_monster_effect(d, a.e(1))
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not e:GetHandler():IsLevelAbove(7) then return false end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_level_above(d, h, 7) {
        return false;
    }
    // local a=Duel.GetAttacker()
    // if not a then return false end
    let Some(atk) = duel::get_attacker(d) else { return false };
    // local d=a:GetBattleTarget()
    let bt = card::get_battle_target(d, atk);
    // if a:IsControler(1-e:GetHandler():GetControler()) then a,d=d,a end
    let h = effect::get_handler(d, a.e).nil();
    let (mut x, mut y) = (Some(atk), bt);
    if card::is_controler(d, atk, 1 - card::get_controler(d, h)) {
        std::mem::swap(&mut x, &mut y);
    }
    // return a and a:IsSetCard(SET_FA)
    match x {
        Some(x) => card::is_set_card(d, x, &[SET_FA as u16]),
        None => false,
    }
}
