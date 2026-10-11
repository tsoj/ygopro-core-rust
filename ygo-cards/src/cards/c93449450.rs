//! F.A. Hang On Mach
use super::prelude::*;

pub const ID: u32 = 93449450;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_FA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // increase ATK
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
    // unaffected
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2a:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e2a, EVENT_CHAIN_SOLVING);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetOperation(s.immop)
    effect::set_operation(d, e2a, Some(operation(immop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.immval)
    effect::set_value_fn(d, e2, lua_fn(immval));
    // e2:SetLabelObject(e2a)
    effect::set_label_object(d, e2, Value::Effect(e2a));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // increase level
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e4, CATEGORY_LVCHANGE);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.lvcon)
    effect::set_condition(d, e4, Some(condition(lvcon)));
    // e4:SetOperation(s.lvop)
    effect::set_operation(d, e4, Some(operation(lvop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // redirect cards
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_RANGE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e5, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e5:SetCode(EFFECT_TO_GRAVE_REDIRECT)
    effect::set_code(d, e5, EFFECT_TO_GRAVE_REDIRECT);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCondition(s.excon)
    effect::set_condition(d, e5, Some(lua_fn(excon)));
    // e5:SetTarget(s.extg)
    effect::set_target(d, e5, Some(effect_card_fn(extg)));
    // e5:SetTargetRange(0,0xff)
    effect::set_target_range(d, e5, 0, 0xff);
    // e5:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e5, LOCATION_REMOVED as i64);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetLevel()*300
    card::get_level(d, c) as i64 * 300
}
/// `s.immop(e,tp,eg,ep,ev,re,r,rp)`
fn immop(d: &mut Duel, a: &EffArgs) {
    // e:SetLabel(e:GetHandler():GetLevel())
    let h = effect::get_handler(d, a.e).nil();
    let lv = card::get_level(d, h);
    effect::set_label(d, a.e, &[lv as i64]);
}
/// `s.immval(e,te)`
fn immval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    // if te:GetOwnerPlayer()~=e:GetHandlerPlayer() and te:IsMonsterEffect() and te:IsActivated() then
    if effect::get_owner_player(d, te) != effect::get_handler_player(d, e)
        && effect::is_monster_effect(d, te)
        && effect::is_activated(d, te)
    {
        // local lv=e:GetLabelObject():GetLabel()
        let e2a = effect::get_label_object(d, e).as_effect().nil();
        let lv = effect::get_label(d, e2a);
        // local tc=te:GetHandler()
        let tc = effect::get_handler(d, te).nil();
        // if tc:GetRank()>0 then
        if card::get_rank(d, tc) > 0 {
            // return tc:GetOriginalRank()<lv
            (card::get_original_rank(d, tc) as i64) < lv
        // elseif tc:HasLevel() then
        } else if card::has_level(d, tc) {
            // return tc:GetOriginalLevel()<lv
            (card::get_original_level(d, tc) as i64) < lv
        // else return false end
        } else {
            false
        }
    // else return false end
    } else {
        false
    }
}
/// `s.lvcon(e,tp,eg,ep,ev,re,r,rp)`
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsSpellTrapEffect() and re:GetHandler():IsSetCard(SET_FA)
    let re = a.re.nil();
    effect::is_spell_trap_effect(d, re) && card::is_set_card(d, effect::get_handler(d, re).nil(), &[SET_FA as u16])
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
        // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.excon(e)`
fn excon(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsLevelAbove(7)
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_level_above(d, h, 7)
}
/// `s.extg(e,c)`
fn extg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetOwner()~=e:GetHandlerPlayer() and Duel.IsPlayerCanRemove(e:GetHandlerPlayer(),c)
    let hp = effect::get_handler_player(d, e);
    card::get_owner(d, c) != hp && duel::is_player_can_remove(d, hp, Some((c, Some(REASON_EFFECT))))
}
