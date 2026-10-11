//! Number 69: Heraldry Crest - Dark Matter Demolition
//! Ｎｏ．６９ 紋章神コート・オブ・アームズ－ゴッド・レイジ
use super::prelude::*;

pub const ID: u32 = 77571454;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_UNKNOWN], xyz_number: Some(69), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 5 Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,5)
    xyz::add_procedure(d, c, None, Some(4), 5, None, None, None, None, false, None);
    // Cannot be destroyed by battle or card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Change opponent's monster's name to "Unknown"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // e3:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e3:SetTarget(s.namechangetg)
    effect::set_target(d, e3, Some(cost_target(namechangetg)));
    // e3:SetOperation(s.namechangeop)
    effect::set_operation(d, e3, Some(operation(namechangeop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Negate the activated effects of "Unknown" your opponent controls
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e4, EVENT_CHAIN_SOLVING);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetOperation(s.disop)
    effect::set_operation(d, e4, Some(operation(disop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.namechangefilter(c)`
fn namechangefilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and not c:IsCode(CARD_UNKNOWN)
    card::is_faceup(d, c) && !card::is_code(d, c, &[CARD_UNKNOWN])
}
/// `s.namechangetg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn namechangetg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and s.namechangefilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && namechangefilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.namechangefilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&namechangefilter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.namechangefilter,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, tp, Some(&namechangefilter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.namechangeop(e,tp,eg,ep,ev,re,r,rp)`
fn namechangeop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and not tc:IsCode(CARD_UNKNOWN) then
    if card::is_relate_to_effect(d, tc, a.e) && !card::is_code(d, tc, &[CARD_UNKNOWN]) {
        // Its name becomes "Unknown"
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_CODE)
        effect::set_code(d, e1, EFFECT_CHANGE_CODE);
        // e1:SetValue(CARD_UNKNOWN)
        effect::set_value(d, e1, CARD_UNKNOWN as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if rp==1-tp and re:IsMonsterEffect() then
    if a.rp == 1 - tp && effect::is_monster_effect(d, a.re.nil()) {
        // local rc=re:GetHandler()
        let rc = effect::get_handler(d, a.re.nil()).nil();
        // if rc:IsRelateToEffect(re) and rc:IsCode(CARD_UNKNOWN) then
        if card::is_relate_to_effect(d, rc, a.re.nil()) && card::is_code(d, rc, &[CARD_UNKNOWN]) {
            // Duel.NegateEffect(ev)
            duel::negate_effect(d, a.ev as u8);
        }
    }
}
