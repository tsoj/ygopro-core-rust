//! S-Force Justify
//! Logical Nonsense and DyXel
use super::prelude::*;

pub const ID: u32 = 35334193;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_S_FORCE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link summon procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),3,3,s.matcheck)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))),
        3,
        Some(3),
        Some(group_check(|d, g, lc, sumtype, tp| matcheck(d, g, lc, sumtype, tp))),
        None,
        None,
    );
    // You cannot summon/set monsters to this card's zones
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_FORCE_MZONE)
    effect::set_code(d, e1, EFFECT_FORCE_MZONE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(function(e)return ~e:GetHandler():GetLinkedZone() end)
    effect::set_value_fn(d, e1, lua_fn(forcezone_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate 1 of opponent's monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish all monsters this card points to
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e3, EVENT_BATTLE_START);
    // e3:SetCondition(s.rmcon)
    effect::set_condition(d, e3, Some(condition(rmcon)));
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.matcheck(g,lc,sumtype,tp)
fn matcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsSetCard,1,nil,SET_S_FORCE,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_set_card_ctx(d, c, &[SET_S_FORCE as u16], Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
// e1:SetValue(function(e)return ~e:GetHandler():GetLinkedZone() end)
fn forcezone_value(d: &mut Duel, a: Args) -> i64 {
    // ~e:GetHandler():GetLinkedZone()
    let c = effect::get_handler(d, a.e(0)).nil();
    !(card::get_linked_zone(d, c, None) as i64)
}
// function s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsNegatableMonster() and chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return card::is_negatable_monster(d, chkc) && card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_negatable_monster(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,Card.IsNegatableMonster,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, c| card::is_negatable_monster(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and not tc:IsDisabled() and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && !card::is_disabled(d, tc) && !card::is_immune_to_effect(d, tc, a.e) {
        // tc:NegateEffects(c,RESET_PHASE|PHASE_END)
        card::negate_effects(d, tc, c, Some(RESET_PHASE | PHASE_END), false, None);
        // if c:IsInExtraMZone(tp) then
        if card::is_in_extra_m_zone(d, c, Some(a.tp)) {
            // local zone=c:GetLinkedZone(1-tp)
            let zone = card::get_linked_zone(d, c, Some(1 - a.tp));
            // nz,filter=Duel.GetLocationCount(1-tp,LOCATION_MZONE,1-tp,LOCATION_REASON_TOFIELD,zone)
            // (nz and filter are globals in the Lua; they are only read here, so they are locals)
            let (nz, filter) = duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(1 - a.tp), Some(LOCATION_REASON_TOFIELD), Some(zone));
            // if nz>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            if nz > 0 && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOZONE as u64);
                // Duel.MoveSequence(tc,math.log(Duel.SelectDisableField(tp,1,0,LOCATION_MZONE,filter<<16)>>16,2))
                let sel = duel::select_disable_field(d, a.tp, 1, 0, LOCATION_MZONE, Some(filter << 16), false);
                if sel == 0 {
                    // nil >> 16 is a Lua error
                    crate::lua_error("attempt to perform bitwise operation on a nil value");
                }
                let seq = ((sel >> 16) as f64).log2() as u8;
                duel::move_sequence(d, tc, seq, None);
            }
        }
    }
}
// function s.rmcon(e,tp,eg,ep,ev,re,r,rp)
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker()==e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    duel::get_attacker(d) == Some(h)
}
// function s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local lg=e:GetHandler():GetLinkedGroup():Filter(Card.IsAbleToRemove,nil)
    let h = effect::get_handler(d, a.e).nil();
    let linked = card::get_linked_group(d, h);
    let lg = group::filter(d, &linked, &|d, c| card::is_able_to_remove(d, c, None, None, None), NO_EX);
    // if chk==0 then return #lg>0 end
    if a.chk == Some(0) {
        return !lg.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,lg,#lg,0,0)
    let n = lg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(lg), n, 0, 0);
    true
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local lg=e:GetHandler():GetLinkedGroup():Filter(Card.IsAbleToRemove,nil)
    let h = effect::get_handler(d, a.e).nil();
    let linked = card::get_linked_group(d, h);
    let lg = group::filter(d, &linked, &|d, c| card::is_able_to_remove(d, c, None, None, None), NO_EX);
    // Duel.Remove(lg,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, lg, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
