//! F.A. Dark Dragster
use super::prelude::*;

pub const ID: u32 = 6764709;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(|d, a: Args| spcon(d, a.e(0), a.card(1)))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --increase ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --increase level
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
    // --destroy 1 card
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_CARD_TARGET, 0);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetTarget(s.rdtg)
    effect::set_target(d, e5, Some(cost_target(rdtg)));
    // e5:SetOperation(s.rdop)
    effect::set_operation(d, e5, Some(operation(rdop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.spfilter(c)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FA) and c:IsLevelAbove(7)
    card::is_set_card(d, c, &[SET_FA as u16]) && card::is_level_above(d, c, 7)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, _e: EffectId, c: Option<CardId>) -> bool {
    // if c==nil then return true end
    let Some(c) = c else { return true };
    // if Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)<=0
    //   or Duel.IsExistingMatchingCard(Card.IsCode,c:GetControler(),LOCATION_MZONE,0,1,nil,id) then return false end
    let tp = card::get_controler(d, c);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        || duel::is_existing_matching_card(d, Some(&|d, x| card::is_code(d, x, &[ID])), tp, LOCATION_MZONE, 0, 1, NO_EX)
    {
        return false;
    }
    // return Duel.IsExistingMatchingCard(s.spfilter,c:GetControler(),LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&spfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetLevel()*300
    card::get_level(d, c) as i64 * 300
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
/// `s.rdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // if chk==0 then return e:GetHandler():GetLevel()>3 and Duel.IsExistingTarget(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::get_level(d, h) > 3 && duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
/// `s.rdop(e,tp,eg,ep,ev,re,r,rp)`
fn rdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not c:IsFacedown() and c:IsRelateToEffect(e) and not c:IsImmuneToEffect(e) and c:UpdateLevel(-3)~=0 and tc and tc:IsRelateToEffect(e) then
    if !card::is_facedown(d, c) && card::is_relate_to_effect(d, c, a.e) && !card::is_immune_to_effect(d, c, a.e) && card::update_level(d, c, -3, None, None) != 0 {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) {
                // Duel.Destroy(tc,REASON_EFFECT)
                duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
            }
        }
    }
}
