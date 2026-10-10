//! Trickstar Light Stage
use super::prelude::*;

pub const ID: u32 = 35371948;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRICKSTAR as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent the activation of a Spell/Trap targeted
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Inflict 200 damage to the opponent when a "Trickstar" monster inflicts damage to them
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e3, EVENT_DAMAGE);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(condition(damcon)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_TRICKSTAR) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_TRICKSTAR as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, &sg, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, &sg);
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_STZONE) and chkc:IsFacedown() end
    if let Some(chkc) = a.chkc() {
        return card::get_controler(d, chkc) == 1 - tp && card::is_location(d, chkc, LOCATION_STZONE) && card::is_facedown(d, chkc);
    }
    // local h=e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFacedown,tp,0,LOCATION_STZONE,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_STZONE, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // Duel.SelectTarget(tp,Card.IsFacedown,tp,0,LOCATION_STZONE,1,1,e:GetHandler())
    duel::select_target(d, tp, Some(&|d, c| card::is_facedown(d, c)), tp, 0, LOCATION_STZONE, 1, 1, false, h.into());
    true
}
/// `s.rcon(e)`
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetOwner():IsHasCardTarget(e:GetHandler()) and e:GetHandler():HasFlagEffect(id)
    let owner = effect::get_owner(d, a.e).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::is_has_card_target(d, owner, h) && card::get_flag_effect(d, h, ID) > 0
}
/// `s.rstcon(e,tp,eg,ep,ev,re,r,rp)`
fn rstcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=e:GetLabelObject():GetLabelObject()
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    let tc = effect::get_label_object(d, e1).as_card().nil();
    let label = effect::get_label(d, a.e);
    // if tc:GetFlagEffectLabel(id)==e:GetLabel()
    //     and c:GetFlagEffectLabel(id)==e:GetLabel() then
    if card::get_flag_effect_label(d, tc, ID).first().copied() == Some(label)
        && card::get_flag_effect_label(d, c, ID).first().copied() == Some(label)
    {
        // return not c:IsDisabled()
        return !card::is_disabled(d, c);
    }
    // e:Reset()
    effect::reset(d, a.e);
    // return false
    false
}
/// `s.rstop(e,tp,eg,ep,ev,re,r,rp)`
fn rstop(d: &mut Duel, a: &EffArgs) {
    // local te=e:GetLabelObject()
    let te = effect::get_label_object(d, a.e).as_effect().nil();
    // te:Reset()
    effect::reset(d, te);
    // Duel.HintSelection(Group.FromCards(e:GetHandler()))
    let h = effect::get_handler(d, a.e).nil();
    duel::hint_selection(d, CardSet::from_card(h), true);
}
/// `s.agcon(e,tp,eg,ep,ev,re,r,rp)`
fn agcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    let label = effect::get_label(d, a.e);
    // if tc:GetFlagEffectLabel(id)==e:GetLabel()
    //     and c:GetFlagEffectLabel(id)==e:GetLabel() then
    if card::get_flag_effect_label(d, tc, ID).first().copied() == Some(label)
        && card::get_flag_effect_label(d, c, ID).first().copied() == Some(label)
    {
        // return not c:IsDisabled()
        return !card::is_disabled(d, c);
    }
    // e:Reset()
    effect::reset(d, a.e);
    // return false
    false
}
/// `s.agop(e,tp,eg,ep,ev,re,r,rp)`
fn agop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.SendtoGrave(tc,REASON_RULE,PLAYER_NONE,1-tp)
    duel::send_to_grave(d, tc, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - a.tp));
}
/// `s.rstop2(e,tp,eg,ep,ev,re,r,rp)`
fn rstop2(d: &mut Duel, a: &EffArgs) {
    // local tc=eg:GetFirst()
    let g = a.eg.clone().nil();
    let tc = group::get_first(d, &g).nil();
    // local c=e:GetOwner()
    let c = effect::get_owner(d, a.e).nil();
    // if tc:GetFlagEffectLabel(id)~=e:GetLabel() or tc==c then return end
    if card::get_flag_effect_label(d, tc, ID).first().copied() != Some(effect::get_label(d, a.e)) || tc == c {
        return;
    }
    // c:CancelCardTarget(tc)
    card::cancel_card_target(d, c, tc);
    // local te=e:GetLabelObject()
    let te = effect::get_label_object(d, a.e).as_effect();
    // tc:ResetFlagEffect(id)
    card::reset_flag_effect(d, tc, ID);
    // if te then te:Reset() end
    if let Some(te) = te {
        effect::reset(d, te);
    }
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc_opt = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsFacedown() and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_facedown(d, tc_opt.nil()) && card::is_relate_to_effect(d, tc_opt.nil(), a.e) {
        let tc = tc_opt.nil();
        // c:SetCardTarget(tc)
        card::set_card_target(d, c, tc);
        // e:SetLabelObject(tc)
        effect::set_label_object(d, a.e, Value::Card(tc));
        // c:ResetFlagEffect(id)
        card::reset_flag_effect(d, c, ID);
        // tc:ResetFlagEffect(id)
        card::reset_flag_effect(d, tc, ID);
        // local fid=c:GetFieldID()
        let fid = card::get_field_id(d, c) as i64;
        // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1,fid)
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, fid, 0);
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1,fid)
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, fid, 0);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_OWNER_RELATE)
        effect::set_property(d, e1, EFFECT_FLAG_OWNER_RELATE, 0);
        // e1:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // e1:SetCondition(s.rcon)
        effect::set_condition(d, e1, Some(condition(rcon)));
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // --End of e1
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
        // e2:SetCountLimit(1)
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // e2:SetLabel(fid)
        effect::set_label(d, e2, &[fid]);
        // e2:SetLabelObject(e1)
        effect::set_label_object(d, e2, Value::Effect(e1));
        // e2:SetCondition(s.rstcon)
        effect::set_condition(d, e2, Some(condition(rstcon)));
        // e2:SetOperation(s.rstop)
        effect::set_operation(d, e2, Some(operation(rstop)));
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
        // --send to grave
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e3:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
        // e3:SetCountLimit(1)
        effect::set_count_limit(d, e3, 1, 0, 0, 0);
        // e3:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
        // e3:SetLabel(fid)
        effect::set_label(d, e3, &[fid]);
        // e3:SetLabelObject(tc)
        effect::set_label_object(d, e3, Value::Card(tc));
        // e3:SetCondition(s.agcon)
        effect::set_condition(d, e3, Some(condition(agcon)));
        // e3:SetOperation(s.agop)
        effect::set_operation(d, e3, Some(operation(agop)));
        // Duel.RegisterEffect(e3,tc:GetControler())
        let tc_controler = card::get_controler(d, tc);
        duel::register_effect(d, e3, tc_controler);
        // --activate check
        // local e4=Effect.CreateEffect(c)
        let e4 = effect::create_effect(d, c);
        // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e4:SetCode(EVENT_CHAINING)
        effect::set_code(d, e4, EVENT_CHAINING);
        // e4:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e4, RESET_PHASE | PHASE_END, 1);
        // e4:SetLabel(fid)
        effect::set_label(d, e4, &[fid]);
        // e4:SetLabelObject(e3)
        effect::set_label_object(d, e4, Value::Effect(e3));
        // e4:SetOperation(s.rstop2)
        effect::set_operation(d, e4, Some(operation(rstop2)));
        // Duel.RegisterEffect(e4,tp)
        duel::register_effect(d, e4, tp);
    }
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not (ep==1-tp and Duel.GetLP(1-tp)>0) then return false end
    if !(a.ep == 1 - a.tp && duel::get_lp(d, 1 - a.tp) > 0) {
        return false;
    }
    // if r&REASON_BATTLE>0 then return eg:GetFirst():IsSetCard(SET_TRICKSTAR) end
    if a.r & REASON_BATTLE > 0 {
        let g = a.eg.clone().nil();
        let tc = group::get_first(d, &g).nil();
        return card::is_set_card(d, tc, &[SET_TRICKSTAR as u16]);
    }
    // return re and re:IsMonsterEffect() and re:IsCardSetcode(SET_TRICKSTAR) and re:IsCardLocation(LOCATION_MZONE) and re:IsCardControler(tp)
    let Some(re) = a.re else {
        return false;
    };
    effect::is_monster_effect(d, re)
        && effect::is_card_setcode(d, re, &[SET_TRICKSTAR as u16])
        && effect::is_card_location(d, re, LOCATION_MZONE)
        && effect::is_card_controler(d, re, a.tp)
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Damage(1-tp,200,REASON_EFFECT)
    duel::damage(d, 1 - a.tp, 200, REASON_EFFECT, false, None);
}
