//! Destiny HERO - Diamond Dude
use super::prelude::*;

pub const ID: u32 = 13093792;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Excavate the top card of your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)==0 then return end
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) == 0 {
        return;
    }
    // Duel.ConfirmDecktop(tp,1)
    duel::confirm_decktop(d, tp, 1);
    // local g=Duel.GetDecktopGroup(tp,1)
    let g = duel::get_decktop_group(d, tp, 1);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // if tc:IsNormalSpell() then
    if card::is_normal_spell(d, tc) {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        // local ae=tc:GetActivateEffect()
        let ae = card::get_activate_effect(d, tc).first().copied();
        // if tc:IsLocation(LOCATION_GRAVE) and ae then
        if card::is_location(d, tc, LOCATION_GRAVE) {
            if let Some(ae) = ae {
                // local e1=Effect.CreateEffect(tc)
                let e1 = effect::create_effect(d, tc);
                // e1:SetDescription(ae:GetDescription())
                effect::set_description(d, e1, effect::get_description(d, ae));
                // e1:SetType(EFFECT_TYPE_IGNITION)
                effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
                // e1:SetCountLimit(1)
                effect::set_count_limit(d, e1, 1, 0, 0, 0);
                // e1:SetRange(LOCATION_GRAVE)
                effect::set_range(d, e1, LOCATION_GRAVE);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_CONTROL|RESET_PHASE|PHASE_END|RESET_SELF_TURN&~RESET_TOFIELD,2)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_CONTROL | RESET_PHASE | PHASE_END | (RESET_SELF_TURN & !RESET_TOFIELD), 2);
                // e1:SetCondition(s.spellcon)
                effect::set_condition(d, e1, Some(condition(spellcon)));
                // e1:SetTarget(s.spelltg)
                effect::set_target(d, e1, Some(cost_target(spelltg)));
                // e1:SetOperation(s.spellop)
                effect::set_operation(d, e1, Some(operation(spellop)));
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc, e1, false);
            }
        }
    } else {
        // Duel.MoveToDeckBottom(tc)
        duel::move_to_deck_bottom(d, tc);
    }
}
// function s.spellcon(e,tp,eg,ep,ev,re,r,rp,chk)
fn spellcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetTurnID()~=Duel.GetTurnCount()
    let h = effect::get_handler(d, a.e).nil();
    card::get_turn_id(d, h) as i32 != duel::get_turn_count(d, None)
}
/// The arguments of an effect function `(e,tp,eg,ep,ev,re,r,rp)` as Lua values.
fn effect_args(a: &EffArgs) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map(Value::Group).unwrap_or(Value::Nil),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        a.re.map(Value::Effect).unwrap_or(Value::Nil),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ]
}
// function s.spelltg(e,tp,eg,ep,ev,re,r,rp,chk)
fn spelltg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    // local ae=e:GetHandler():GetActivateEffect()
    let h = effect::get_handler(d, e).nil();
    let ae = card::get_activate_effect(d, h).first().copied().nil();
    // local ftg=ae:GetTarget()
    let ftg = effect::get_target(d, ae);
    // if chk==0 then
    if a.chk == Some(0) {
        // return not ftg or ftg(e,tp,eg,ep,ev,re,r,rp,chk)
        return match ftg {
            None => true,
            Some(f) => {
                let mut args = effect_args(a);
                args.push(Value::Int(0));
                f.call(d, &args).truthy()
            }
        };
    }
    // if ae:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then
    if effect::is_has_property(d, ae, EFFECT_FLAG_CARD_TARGET, 0) {
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
    } else {
        // else e:SetProperty(0) end
        effect::set_property(d, e, 0, 0);
    }
    // if ftg then
    if let Some(f) = ftg {
        // ftg(e,tp,eg,ep,ev,re,r,rp,chk)
        let mut args = effect_args(a);
        args.push(Value::Int(a.chk.unwrap_or(1)));
        f.call(d, &args);
    }
    true
}
// function s.spellop(e,tp,eg,ep,ev,re,r,rp)
fn spellop(d: &mut Duel, a: &EffArgs) {
    // local ae=e:GetHandler():GetActivateEffect()
    let h = effect::get_handler(d, a.e).nil();
    let ae = card::get_activate_effect(d, h).first().copied().nil();
    // local fop=ae:GetOperation()
    let fop = effect::get_operation(d, ae).nil();
    // fop(e,tp,eg,ep,ev,re,r,rp)
    fop.call(d, &effect_args(a));
}
