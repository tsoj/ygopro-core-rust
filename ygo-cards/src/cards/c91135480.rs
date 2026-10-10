//! Time Thief Double Barrel
use super::prelude::*;

pub const ID: u32 = 91135480;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz summon procedure
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // Detach and apply multiple effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_CONTROL+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_CONTROL + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(_,tp,_,ep)return ep==1-tp end)
    effect::set_condition(d, e1, Some(condition(|_d: &mut Duel, a: &EffArgs| a.ep == 1 - a.tp)));
    // e1:SetTarget(s.tg)
    effect::set_target(d, e1, Some(cost_target(tg)));
    // e1:SetOperation(s.op)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsMonster,nil)<=1
    //   and sg:FilterCount(Card.IsSpell,nil)<=1
    //   and sg:FilterCount(Card.IsTrap,nil)<=1
    let r = group::filter_count(d, sg, &|d, x| card::is_monster(d, x), NO_EX) <= 1
        && group::filter_count(d, sg, &|d, x| card::is_spell(d, x), NO_EX) <= 1
        && group::filter_count(d, sg, &|d, x| card::is_trap(d, x), NO_EX) <= 1;
    (r, false)
}
/// The card types that can be negated or taken: `ty` in `s.tg` and `s.op`.
fn ty_of(d: &mut Duel, c: CardId, tp: u8) -> u32 {
    // local ty=0
    let mut ty = 0;
    // if c:IsFaceup() then ty=ty|TYPE_MONSTER end
    if card::is_faceup(d, c) {
        ty |= TYPE_MONSTER;
    }
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsControlerCanBeChanged),tp,0,LOCATION_MZONE,1,nil) then ty=ty|TYPE_SPELL end
    if duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_controler_can_be_changed(d, x, false, None)), tp, 0, LOCATION_MZONE, 1, NO_EX) {
        ty |= TYPE_SPELL;
    }
    // if Duel.IsExistingMatchingCard(Card.IsNegatableMonster,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) then ty=ty|TYPE_TRAP end
    if duel::is_existing_matching_card(d, Some(&|d, x| card::is_negatable_monster(d, x)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX) {
        ty |= TYPE_TRAP;
    }
    ty
}
/// `s.tg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=c:GetOverlayGroup()
    let g = card::get_overlay_group(d, c);
    // local ty=0 ... (see ty_of)
    let ty = ty_of(d, c, a.tp);
    // if chk==0 then return ty>0 and g:IsExists(Card.IsType,1,nil,ty) end
    if a.chk == Some(0) {
        return ty > 0 && group::is_exists(d, &g, &|d, x| card::is_type(d, x, ty), 1, NO_EX);
    }
    true
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=c:GetOverlayGroup()
    let g = card::get_overlay_group(d, c);
    // local ty=0 ... (see ty_of)
    let ty = ty_of(d, c, tp);
    // if ty==0 then return end
    if ty == 0 {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g:Filter(Card.IsType,nil,ty),e,tp,1,3,s.rescon,1,tp,HINTMSG_XMATERIAL)
    let fg = group::filter(d, &g, &|d, x| card::is_type(d, x, ty), NO_EX);
    let sg = aux::select_unselect_group(d, &fg, Some(a.e), tp, Some(1), Some(3), Some(&rescon), 1, tp, Some(HINTMSG_XMATERIAL as u64), None, None, false)
        .unwrap_or_default();
    // local card_type=0
    let mut card_type = 0;
    // for tc in sg:Iter() do card_type=card_type|tc:GetMainCardType() end
    for tc in group::iter(d, &sg) {
        card_type |= card::get_main_card_type(d, tc);
    }
    // Duel.SendtoGrave(sg,REASON_EFFECT)
    duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    // Duel.RaiseSingleEvent(c,EVENT_DETACH_MATERIAL,e,0,0,0,0)
    duel::raise_single_event(d, c, EVENT_DETACH_MATERIAL, Some(a.e), 0, 0, 0, 0);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if (card_type&TYPE_MONSTER)>0 then
    if (card_type & TYPE_MONSTER) > 0 {
        // if c:IsFaceup() and c:IsRelateToEffect(e) then
        if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT)
            effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
            // e1:SetValue(400)
            effect::set_value(d, e1, 400);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
    // if (card_type&TYPE_SPELL)>0 then
    if (card_type & TYPE_SPELL) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
        // local tc=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsControlerCanBeChanged),tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, x| card::is_faceup(d, x) && card::is_controler_can_be_changed(d, x, false, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel);
        // if tc and Duel.GetControl(tc,tp,PHASE_END,1)~=0 then
        if let Some(tc) = tc {
            if duel::get_control(d, tc, tp, PHASE_END as u16, 1, None, None) != 0 {
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, h);
                // --Cannot attack
                // e1:SetDescription(3206)
                effect::set_description(d, e1, 3206);
                // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
                effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_CANNOT_ATTACK)
                effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
                // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TURN_SET))
                effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET), 1);
                // e1:SetValue(1)
                effect::set_value(d, e1, 1);
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc, e1, false);
                // --Cannot activate its effects
                // local e2=e1:Clone()
                let e2 = effect::clone(d, e1);
                // e2:SetDescription(3302)
                effect::set_description(d, e2, 3302);
                // e2:SetCode(EFFECT_CANNOT_TRIGGER)
                effect::set_code(d, e2, EFFECT_CANNOT_TRIGGER);
                // tc:RegisterEffect(e2)
                card::register_effect(d, tc, e2, false);
            }
        }
    }
    // if (card_type&TYPE_TRAP)>0 then
    if (card_type & TYPE_TRAP) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsNegatableMonster,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&|d, x| card::is_negatable_monster(d, x)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // --Negate effects of a monster
            // local tc=g:GetFirst()
            let tc = group::get_first(d, &g).nil();
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(e:GetHandler())
            let e2 = effect::create_effect(d, h);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetValue(RESET_TURN_SET)
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            // e2:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
