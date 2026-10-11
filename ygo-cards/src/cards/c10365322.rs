//! Ignition Beast Volcannon
use super::prelude::*;

pub const ID: u32 = 10365322;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsRace,RACE_MACHINE),aux.FilterBoolFunctionEx(Card.IsRace,RACE_PYRO))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_MACHINE, Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_PYRO, Some(fc), st, Some(tp)))),
        ],
    );
    // Destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsFusionSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.TRUE,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,0,LOCATION_MZONE,1,1,nil)
    let mut g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // g:AddCard(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    group::add_card(&mut g, &CardSet::from_card(h));
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (tc:IsRelateToEffect(e) and c:IsRelateToEffect(e)) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_relate_to_effect(d, c, a.e)) {
        return;
    }
    // if not tc:IsControler(1-tp) then return end
    if !card::is_controler(d, tc, 1 - a.tp) {
        return;
    }
    // local dg=Group.FromCards(c,tc)
    let mut dg = CardSet::from_card(c);
    group::add_card(&mut dg, &CardSet::from_card(tc));
    // if Duel.Destroy(dg,REASON_EFFECT)==2 and tc:IsLocation(LOCATION_GRAVE) and c:IsLocation(LOCATION_GRAVE) then
    if duel::destroy(d, dg, REASON_EFFECT, None, None) == 2 && card::is_location(d, tc, LOCATION_GRAVE) && card::is_location(d, c, LOCATION_GRAVE) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local atk=tc:GetAttack()
        let mut atk = card::get_attack(d, tc);
        // if atk<0 then atk=0 end
        if atk < 0 {
            atk = 0;
        }
        // Duel.Damage(1-tp,atk,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, atk as i64, REASON_EFFECT, false, None);
    }
}
