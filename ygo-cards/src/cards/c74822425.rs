//! El Shaddoll Shekhinaga
use super::prelude::*;

pub const ID: u32 = 74822425;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SHADDOLL as u32], material_setcode: &[SET_SHADDOLL as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_SHADDOLL),s.matfilter)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_set_card_ctx(d, x, &[SET_SHADDOLL as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| matfilter(d, x, fc, st, tp))),
        ],
    );
    // cannot spsummon
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e2, EFFECT_SPSUMMON_CONDITION);
    effect::set_range(d, e2, LOCATION_EXTRA);
    effect::set_value_fn(d, e2, aux::fuslimit());
    card::register_effect(d, c, e2, false);
    // negate
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_code(d, e3, EVENT_CHAINING);
    effect::set_condition(d, e3, Some(condition(discon)));
    effect::set_target(d, e3, Some(cost_target(distg)));
    effect::set_operation(d, e3, Some(operation(disop)));
    card::register_effect(d, c, e3, false);
    // tohand
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_TOHAND);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_target(d, e4, Some(cost_target(thtg)));
    effect::set_operation(d, e4, Some(operation(thop)));
    card::register_effect(d, c, e4, false);
}
/// `s.matfilter(c,lc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH,lc,sumtype,tp) or c:IsHasEffect(4904633)
    card::is_attribute_ctx(d, c, ATTRIBUTE_EARTH, Some(lc), sumtype, Some(tp)) || !card::is_has_effect(d, c, 4904633, None).is_empty()
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // if e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) then return false end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_status(d, h, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    // local loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION]).first().map_or(0, |v| v.int());
    // return loc==LOCATION_MZONE and re:IsMonsterEffect()
    //     and (re:GetHandler():GetSummonType()&SUMMON_TYPE_SPECIAL)==SUMMON_TYPE_SPECIAL
    //     and Duel.IsChainNegatable(ev)
    loc == LOCATION_MZONE as i64
        && effect::is_monster_effect(d, a.re.nil())
        && {
            let rh = effect::get_handler(d, a.re.nil()).nil();
            (card::get_summon_type(d, rh) & SUMMON_TYPE_SPECIAL) == SUMMON_TYPE_SPECIAL
        }
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsSetCard,tp,LOCATION_HAND,0,1,nil,SET_SHADDOLL) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_set_card(d, c, &[SET_SHADDOLL as u16])), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsRelateToEffect(re) and re:GetHandler():IsDestructable() then
    let rh = effect::get_handler(d, a.re.nil()).nil();
    if card::is_relate_to_effect(d, rh, a.re.nil()) && card::is_destructable(d, rh, None) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) and Duel.Destroy(eg,REASON_EFFECT)>0 then
    if duel::negate_activation(d, a.ev as u8) {
        let rh = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, rh, a.re.nil()) && duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None) > 0 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
            // local g=Duel.SelectMatchingCard(tp,Card.IsSetCard,tp,LOCATION_HAND,0,1,1,nil,SET_SHADDOLL)
            let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_set_card(d, c, &[SET_SHADDOLL as u16])), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SendtoGrave(g,REASON_EFFECT)
                duel::send_to_grave(d, g, REASON_EFFECT, None, None);
            }
        }
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SHADDOLL) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_SHADDOLL as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None);
        }
    }
}
