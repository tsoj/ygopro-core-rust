//! Number 84: Pain Gainer
use super::prelude::*;

pub const ID: u32 = 26556950;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(84), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
    // Xyz.AddProcedure(c,nil,11,2,s.ovfilter,aux.Stringid(id,0),2)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(11),
        2,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(2)),
        None,
        false,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Gain DEF x200 for all combined Ranks
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e1, EFFECT_UPDATE_DEFENSE);
    // e1:SetValue(s.defval)
    effect::set_value_fn(d, e1, effect_card_value(defval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Inflict 600 damage when opponent activates a Spell/Trap
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e3, EVENT_CHAIN_SOLVED);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(condition(damcon)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy face-up monsters with DEF less than this card
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e4, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.ovfilter(c,tp,xyzc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // local rk=c:GetRank()
    let rk = card::get_rank(d, c);
    // return c:IsFaceup() and c:GetOverlayCount()>=2 and c:IsType(TYPE_XYZ,xyzc,SUMMON_TYPE_XYZ,tp) and c:IsAttribute(ATTRIBUTE_DARK,xyzc,SUMMON_TYPE_XYZ,tp) and rk>=8 and rk<=10
    card::is_faceup(d, c)
        && card::get_overlay_count(d, c) >= 2
        && card::is_type_ctx(d, c, TYPE_XYZ, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && rk >= 8
        && rk <= 10
}
/// `s.defval(e,c)`
fn defval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,c:GetControler(),LOCATION_MZONE,0,nil)
    let p = card::get_controler(d, c);
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), p, LOCATION_MZONE, 0, NO_EX);
    // return g:GetSum(Card.GetRank)*200
    group::get_sum(d, &g, &|d, x| card::get_rank(d, x) as i64) * 200
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // if rp==tp or not re:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if a.rp == a.tp || !effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET)|RESET_CHAIN,0,1)
    let c = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET) | RESET_CHAIN, 0, 1, 0, 0);
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:GetOverlayCount()>0 and ep~=tp and c:GetFlagEffect(id)~=0 and re:IsHasType(EFFECT_TYPE_ACTIVATE)
    card::get_overlay_count(d, c) > 0
        && a.ep != a.tp
        && card::get_flag_effect(d, c, ID) != 0
        && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE)
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Damage(1-tp,600,REASON_EFFECT)
    duel::damage(d, 1 - a.tp, 600, REASON_EFFECT, false, None);
}
/// `s.desfilter(c,def)`
fn desfilter(d: &mut Duel, c: CardId, def: i32) -> bool {
    // return c:IsFaceup() and c:IsDefenseBelow(def)
    card::is_faceup(d, c) && card::is_defense_below(d, c, def)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.desfilter,tp,0,LOCATION_MZONE,1,nil,c:GetDefense()) end
    if a.chk == Some(0) {
        let def = card::get_defense(d, c);
        return duel::is_existing_matching_card(d, Some(&|d, x| desfilter(d, x, def)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil,c:GetDefense())
    let def = card::get_defense(d, c);
    let g = duel::get_matching_group(d, Some(&|d, x| desfilter(d, x, def)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = group::get_count(&g);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n as u8, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil,c:GetDefense())
    let def = card::get_defense(d, c);
    let g = duel::get_matching_group(d, Some(&|d, x| desfilter(d, x, def)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
