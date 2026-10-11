//! Gigathunder Giclops
use super::prelude::*;

pub const ID: u32 = 17241941;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Enable pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Change battle position of 1 monster from each side
    let e0 = effect::create_effect(d, c);
    // e0:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e0, CATEGORY_POSITION);
    // e0:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e0, EFFECT_TYPE_IGNITION);
    // e0:SetRange(LOCATION_PZONE)
    effect::set_range(d, e0, LOCATION_PZONE);
    // e0:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e0, EFFECT_FLAG_CARD_TARGET, 0);
    // e0:SetCountLimit(1)
    effect::set_count_limit(d, e0, 1, 0, 0, 0);
    // e0:SetTarget(s.pentg)
    effect::set_target(d, e0, Some(cost_target(pentg)));
    // e0:SetOperation(s.penop)
    effect::set_operation(d, e0, Some(operation(penop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Banish itself if used as synchro material
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
    effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e1, LOCATION_REMOVED as i64);
    // e1:SetCondition(s.rmcon)
    effect::set_condition(d, e1, Some(condition(rmcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If normal summoned, change its battle position
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e2, CATEGORY_POSITION);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetOperation(s.posop)
    effect::set_operation(d, e2, Some(operation(posop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy 1 face-up spell/trap on the field
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_CHANGE_POS)
    effect::set_code(d, e3, EVENT_CHANGE_POS);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Return 1 card in your pendulum zone to hand
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e4, CATEGORY_TOHAND);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e4:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e4, 1, ID, 2, 0);
    // e4:SetCondition(s.sendcon)
    effect::set_condition(d, e4, Some(condition(sendcon)));
    // e4:SetTarget(s.sendtg)
    effect::set_target(d, e4, Some(cost_target(sendtg)));
    // e4:SetOperation(s.sendop)
    effect::set_operation(d, e4, Some(operation(sendop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.penfilter(c)
fn penfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCanChangePosition() and c:IsType(TYPE_PENDULUM)
    card::is_faceup(d, c) && card::is_can_change_position(d, c) && card::is_type(d, c, TYPE_PENDULUM)
}
// function s.facedown_pos_filter(c): aux.FaceupFilter(Card.IsCanChangePosition)
fn faceup_can_change(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_can_change_position(d, c)
}
// function s.pentg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.penfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsCanChangePosition),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&penfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&faceup_can_change), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    // local g1=Duel.SelectTarget(tp,s.penfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let mut g1 = duel::select_target(d, tp, Some(&penfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    // local g2=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsCanChangePosition),tp,0,LOCATION_MZONE,1,1,nil)
    let g2 = duel::select_target(d, tp, Some(&faceup_can_change), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g1,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g1), 2, 0, 0);
    true
}
// function s.penop(e,tp,eg,ep,ev,re,r,rp)
fn penop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 then
    if g.len() > 0 {
        // Duel.ChangePosition(g,POS_FACEUP_DEFENSE,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK)
        duel::change_position(
            d,
            g,
            POS_FACEUP_DEFENSE as u8,
            Some(POS_FACEUP_ATTACK as u8),
            Some(POS_FACEUP_ATTACK as u8),
            Some(POS_FACEUP_ATTACK as u8),
            false,
        );
    }
}
// function s.rmcon(e)
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsSummonLocation(LOCATION_EXTRA)
    //     and (c:GetReason()&REASON_MATERIAL+REASON_SYNCHRO)==REASON_MATERIAL+REASON_SYNCHRO
    let mat_syn = REASON_MATERIAL + REASON_SYNCHRO;
    card::is_summon_location(d, c, LOCATION_EXTRA) && (card::get_reason(d, c) & mat_syn) == mat_syn
}
// function s.posop(e,tp,eg,ep,ev,re,r,rp)
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.ChangePosition(c,POS_FACEUP_DEFENSE,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK)
        duel::change_position(
            d,
            c,
            POS_FACEUP_DEFENSE as u8,
            Some(POS_FACEUP_ATTACK as u8),
            Some(POS_FACEUP_ATTACK as u8),
            Some(POS_FACEUP_ATTACK as u8),
            false,
        );
    }
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSpellTrap()
    card::is_faceup(d, c) && card::is_spell_trap(d, c)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
// function s.sendcon(e,tp,eg,ep,ev,re,r,rp)
fn sendcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_PZONE)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_PZONE)
}
// function s.sendtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sendtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_PZONE,0)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, LOCATION_PZONE, 0) > 0;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_PZONE,0)
    let g = duel::get_field_group(d, tp, LOCATION_PZONE, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, 0, 0);
    true
}
// function s.sendop(e,tp,eg,ep,ev,re,r,rp)
fn sendop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.GetFieldGroup(tp,LOCATION_PZONE,0):Select(tp,1,1,nil)
    let pz = duel::get_field_group(d, tp, LOCATION_PZONE, 0);
    let g = group::select(d, &pz, tp, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    // if not tc then return end
    let Some(tc) = group::get_first(d, &g) else { return };
    // Duel.HintSelection(g)
    duel::hint_selection(d, g, true);
    // Duel.SendtoHand(tc,nil,REASON_EFFECT)
    duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
}
