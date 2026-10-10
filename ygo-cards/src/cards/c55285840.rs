//! Time Thief Redoer
use super::prelude::*;

pub const ID: u32 = 55285840;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // Attach top deck card during the Standby Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.xyztg)
    effect::set_target(d, e1, Some(cost_target(xyztg)));
    // e1:SetOperation(s.xyzop)
    effect::set_operation(d, e1, Some(operation(xyzop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Detach and apply multiple effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_REMOVE+CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_REMOVE + CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.tg)
    effect::set_target(d, e2, Some(cost_target(tg)));
    // e2:SetOperation(s.op)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.xyztg(e,tp,...,chk)`
fn xyztg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsType(TYPE_XYZ) and Duel.GetFieldGroupCount(1-tp,LOCATION_DECK,0)>0 end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return card::is_type(d, c, TYPE_XYZ) && duel::get_field_group_count(d, 1 - a.tp, LOCATION_DECK, 0) > 0;
    }
    true
}
/// `s.xyzop(e,tp,...)`
fn xyzop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetDecktopGroup(1-tp,1)
    let g = duel::get_decktop_group(d, 1 - a.tp, 1);
    // if c:IsRelateToEffect(e) and #g==1 then
    if card::is_relate_to_effect(d, c, a.e) && g.len() == 1 {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.Overlay(c,g)
        duel::overlay(d, c, g, false);
    }
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsMonster,nil)<=1
    //     and sg:FilterCount(Card.IsSpell,nil)<=1
    //     and sg:FilterCount(Card.IsTrap,nil)<=1
    let monsters = group::filter(d, sg, &card::is_monster, NO_EX).len();
    let spells = group::filter(d, sg, &card::is_spell, NO_EX).len();
    let traps = group::filter(d, sg, &card::is_trap, NO_EX).len();
    (monsters <= 1 && spells <= 1 && traps <= 1, false)
}
/// The card types of the materials this card can send (`ty` in `s.tg` and `s.op`).
fn send_types(d: &mut Duel, c: CardId, tp: u8) -> u32 {
    // local ty=0
    let mut ty = 0;
    // if c:IsAbleToRemove() then ty=ty|TYPE_MONSTER end
    if card::is_able_to_remove(d, c, None, None, None) {
        ty |= TYPE_MONSTER;
    }
    // if Duel.IsPlayerCanDraw(tp,1) then ty=ty|TYPE_SPELL end
    if duel::is_player_can_draw(d, tp, 1) {
        ty |= TYPE_SPELL;
    }
    // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAbleToDeck),tp,0,LOCATION_ONFIELD,1,nil) then ty=ty|TYPE_TRAP end
    if duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_able_to_deck(d, x))), tp, 0, LOCATION_ONFIELD, 1, NO_EX) {
        ty |= TYPE_TRAP;
    }
    ty
}
/// `s.tg(e,tp,...,chk)`
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=c:GetOverlayGroup()
    let g = card::get_overlay_group(d, c);
    // (ty computed as in s.op)
    let ty = send_types(d, c, a.tp);
    // if chk==0 then return ty>0 and g:IsExists(Card.IsType,1,nil,ty) end
    if a.chk == Some(0) {
        return ty > 0 && group::is_exists(d, &g, &|d, x| card::is_type(d, x, ty), 1, NO_EX);
    }
    true
}
/// `s.op(e,tp,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=c:GetOverlayGroup()
    let g = card::get_overlay_group(d, c);
    // local ty=0 ... (see send_types)
    let ty = send_types(d, c, a.tp);
    // if ty==0 then return end
    if ty == 0 {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g:Filter(Card.IsType,nil,ty),e,tp,1,3,s.rescon,1,tp,HINTMSG_REMOVEXYZ)
    let filtered = group::filter(d, &g, &|d, x| card::is_type(d, x, ty), NO_EX);
    let sg = aux::select_unselect_group(d, &filtered, Some(a.e), a.tp, Some(1), Some(3), Some(&rescon), 1, a.tp, Some(HINTMSG_REMOVEXYZ as u64), None, None, false)
        .unwrap_or_default();
    // local lb=0
    let mut lb: u32 = 0;
    // for tc in aux.Next(sg) do lb=lb|tc:GetMainCardType() end
    for tc in group::iter(d, &sg) {
        lb |= card::get_main_card_type(d, tc);
    }
    // Duel.SendtoGrave(sg,REASON_EFFECT)
    duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    // Duel.RaiseSingleEvent(c,EVENT_DETACH_MATERIAL,e,0,0,0,0)
    duel::raise_single_event(d, c, EVENT_DETACH_MATERIAL, Some(a.e), 0, 0, 0, 0);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if lb & TYPE_MONSTER ~=0 then
    if lb & TYPE_MONSTER != 0 {
        // if c:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, c, a.e) {
            // Duel.Remove(c,c:GetPosition(),REASON_EFFECT|REASON_TEMPORARY)
            let pos = card::get_position(d, c);
            duel::remove(d, c, pos as u8, REASON_EFFECT | REASON_TEMPORARY, None, None);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
            // e1:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            // e1:SetLabelObject(c)
            effect::set_label_object(d, e1, Value::Card(c));
            // e1:SetCountLimit(1)
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            // e1:SetOperation(s.retop)
            effect::set_operation(d, e1, Some(operation(retop)));
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, a.tp);
        }
    }
    // if lb & TYPE_SPELL ~=0 then Duel.Draw(tp,1,REASON_EFFECT) end
    if lb & TYPE_SPELL != 0 {
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    }
    // if lb & TYPE_TRAP ~=0 then
    if lb & TYPE_TRAP != 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsAbleToDeck),tp,0,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_matching_card(
            d,
            a.tp,
            Some(&aux::faceup_filter(|d, x| card::is_able_to_deck(d, x))),
            a.tp,
            0,
            LOCATION_ONFIELD,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 then Duel.SendtoDeck(g,nil,SEQ_DECKTOP,REASON_EFFECT) end
        if !g.is_empty() {
            duel::send_to_deck(d, g, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
        }
    }
}
/// `s.retop(e,tp,...)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // Duel.ReturnToField(e:GetLabelObject())
    let c = effect::get_label_object(d, a.e).as_card().nil();
    duel::return_to_field(d, c, None, None);
}
