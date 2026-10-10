//! Harr, Generaider Boss of Storms
use super::prelude::*;

pub const ID: u32 = 68199168;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GENERAIDER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Make the opponent send 1 monster to the GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_TO_HAND);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(scon0)));
    effect::set_target(d, e1, Some(cost_target(stg0)));
    effect::set_operation(d, e1, Some(operation(sop0)));
    card::register_effect(d, c, e1, false);
    // Negate an effect's activation
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(discon)));
    effect::set_cost(d, e2, Some(cost_target(discost)));
    effect::set_target(d, e2, Some(cost_target(distg)));
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsPreviousLocation(LOCATION_DECK)
    card::is_controler(d, c, tp) && card::is_previous_location(d, c, LOCATION_DECK)
}
/// `s.scon0(e,tp,eg,ep,ev,re,r,rp)`
fn scon0(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetCurrentPhase()~=PHASE_DRAW and eg:IsExists(s.cfilter,1,nil,1-tp)
    //     and Duel.GetFieldGroupCount(1-tp,LOCATION_ONFIELD|LOCATION_HAND,0)>0
    if duel::get_current_phase(d) == PHASE_DRAW {
        return false;
    }
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, 1 - a.tp), 1, NO_EX)
        && duel::get_field_group_count(d, 1 - a.tp, LOCATION_ONFIELD | LOCATION_HAND, 0) > 0
}
/// `s.stg0(e,tp,eg,ep,ev,re,r,rp,chk)`
fn stg0(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(1-tp,LOCATION_MZONE|LOCATION_HAND,0)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, 1 - a.tp, LOCATION_MZONE | LOCATION_HAND, 0) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,0,LOCATION_MZONE|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 0, (LOCATION_MZONE | LOCATION_HAND) as i32);
    true
}
/// `s.sop0(e,tp,eg,ep,ev,re,r,rp)`
fn sop0(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(1-tp,Card.IsMonster,1-tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, 1 - a.tp, Some(&card::is_monster), 1 - a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SendtoGrave(g,REASON_RULE,PLAYER_NONE,1-tp) end
    if !g.is_empty() {
        duel::send_to_grave(d, g, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - a.tp));
    }
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsChainNegatable(ev)
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.disfilter(c,tp)`
fn disfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (c:IsRace(RACE_SPELLCASTER) or (c:IsSetCard(SET_GENERAIDER) and c:IsMonster())) and (c:IsControler(tp) or c:IsFaceup())
    (card::is_race(d, c, RACE_SPELLCASTER) || (card::is_set_card(d, c, &[SET_GENERAIDER as u16]) && card::is_monster(d, c)))
        && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.disfilter,2,false,nil,nil,tp) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(d, a.tp, Some(&|d, c| disfilter(d, c, a.tp)), 2, 2, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.disfilter,2,2,false,nil,nil,tp)
    let g = aux::select_release_group_cost(d, a.tp, Some(&|d, c| disfilter(d, c, a.tp)), 2, 2, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
