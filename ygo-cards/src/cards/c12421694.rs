//! Sky Striker Ace - Kaina
use super::prelude::*;

pub const ID: u32 = 12421694;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_SKY_STRIKER_ACE, SET_SKY_STRIKER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Can only Special Summon "Sky Striker Ace - Kaina" once per turn
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // Link Summon procedure
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| matfilter(d, c, sc, st, tp))), 1, Some(1), None, None, None);
    // Targeted monster cannot attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetTarget(s.natg)
    effect::set_target(d, e1, Some(cost_target(natg)));
    // e1:SetOperation(s.naop)
    effect::set_operation(d, e1, Some(operation(naop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gain 100 LP each time you activate a "Sky Striker" Spell or effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetOperation(aux.chainreg)
    effect::set_operation(d, e2, Some(aux::chainreg()));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e3, EVENT_CHAIN_SOLVED);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(function(e) return e:GetHandler():HasFlagEffect(1) end)
    effect::set_condition(
        d,
        e3,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::has_flag_effect(d, h, 1, None)
        })),
    );
    // e3:SetOperation(s.lpop)
    effect::set_operation(d, e3, Some(operation(lpop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.matfilter(c,scard,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, sc: Option<CardId>, st: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_SKY_STRIKER_ACE,scard,sumtype,tp) and c:IsAttributeExcept(ATTRIBUTE_EARTH,scard,sumtype,tp)
    card::is_set_card_ctx(d, c, &[SET_SKY_STRIKER_ACE as u16], sc, st, Some(tp)) && card::is_attribute_except(d, c, ATTRIBUTE_EARTH, sc, st, Some(tp))
}
/// `s.natg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn natg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.naop(e,tp,eg,ep,ev,re,r,rp)`
fn naop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local ct=Duel.IsTurnPlayer(tp) and 2 or 1
        let ct: u16 = if duel::is_turn_player(d, tp) { 2 } else { 1 };
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // --Cannot attack
        // e1:SetDescription(3206)
        effect::set_description(d, e1, 3206);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END,ct)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, ct);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.lpop(e,tp,eg,ep,ev,re,r,rp)`
fn lpop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if re:IsSpellEffect() and rp==tp and re:GetHandler():IsSetCard(SET_SKY_STRIKER) then
    let re = a.re.nil();
    if effect::is_spell_effect(d, re) && a.rp == tp {
        let rc = effect::get_handler(d, re).nil();
        if card::is_set_card(d, rc, &[SET_SKY_STRIKER as u16]) {
            // Duel.Hint(HINT_CARD,0,id)
            duel::hint(d, HINT_CARD as u8, 0, ID as u64);
            // Duel.Recover(tp,100,REASON_EFFECT)
            duel::recover(d, tp, 100, REASON_EFFECT, false, None);
        }
    }
}
