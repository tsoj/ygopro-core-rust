//! Haggard Lizardose
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 9763474;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 2 monsters with different names
    // Link.AddProcedure(c,nil,2,2,function(g) return g:GetClassCount(Card.GetCode)==#g end)
    link::add_procedure(
        d,
        c,
        None,
        2,
        Some(2),
        Some(group_check(|d, g, _sc, _st, _tp| group::get_class_count(d, g, &|d, cc| card::get_code(d, cc)[0] as i64) == g.len())),
        None,
        None,
    );
    // Make 1 monster's ATK equal to banished monster's ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.atkcost)
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn atkcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local atk=c:GetTextAttack()
    let mut atk = card::get_text_attack(d, c);
    // if atk<0 then atk=0 end
    if atk < 0 {
        atk = 0;
    }
    // return c:IsFaceup() and c:IsAttackBelow(2000) and c:IsAbleToRemoveAsCost() and (not c:IsOriginalRace(RACE_REPTILE) or Duel.IsPlayerCanDraw(tp,1))
    //     and Duel.IsExistingTarget(aux.FaceupFilter(aux.NOT(Card.IsAttack),atk),tp,LOCATION_MZONE,LOCATION_MZONE,1,c)
    card::is_faceup(d, c)
        && card::is_attack_below(d, c, 2000)
        && card::is_able_to_remove_as_cost(d, c, None)
        && (!card::is_original_race(d, c, RACE_REPTILE) || duel::is_player_can_draw(d, tp, 1))
        && duel::is_existing_target(
            d,
            Some(&|d, x| card::is_faceup(d, x) && !card::is_attack(d, x, &[atk as u32])),
            tp,
            LOCATION_MZONE,
            LOCATION_MZONE,
            1,
            c.into(),
        )
}
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.atkcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| atkcostfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.atkcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| atkcostfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // e:SetLabel(tc:GetTextAttack(),tc:GetOriginalRace())
    let atk = card::get_text_attack(d, tc);
    let race = card::get_original_race(d, tc);
    effect::set_label(d, a.e, &[atk as i64, race as i64]);
    // Duel.Remove(tc,POS_FACEUP,REASON_COST)
    duel::remove(d, tc, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local atk,race=e:GetLabel()
    let labels = effect::get_labels(d, e);
    let mut atk = labels.first().copied().unwrap_or(0) as i32;
    let race = labels.get(1).copied().unwrap_or(0) as u64;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and not chkc:IsAttack(atk) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc) && !card::is_attack(d, chkc, &[atk as u32]);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if atk<0 then atk=0 end
    if atk < 0 {
        atk = 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
    // local g=Duel.SelectTarget(tp,aux.FaceupFilter(aux.NOT(Card.IsAttack),atk),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let _g = duel::select_target(
        d,
        tp,
        Some(&|d, c| card::is_faceup(d, c) && !card::is_attack(d, c, &[atk as u32])),
        tp,
        LOCATION_MZONE,
        LOCATION_MZONE,
        1,
        1,
        false,
        NO_EX,
    );
    // if race&RACE_REPTILE==0 then
    if race & RACE_REPTILE == 0 {
        // e:SetCategory(CATEGORY_ATKCHANGE)
        effect::set_category(d, e, CATEGORY_ATKCHANGE);
    } else {
        // e:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DRAW)
        effect::set_category(d, e, CATEGORY_ATKCHANGE + CATEGORY_DRAW);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    }
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc)) {
        return;
    }
    // local atk,race=e:GetLabel()
    let labels = effect::get_labels(d, e);
    let mut atk = labels.first().copied().unwrap_or(0) as i32;
    let race = labels.get(1).copied().unwrap_or(0) as u64;
    // if atk<0 then atk=0 end
    if atk < 0 {
        atk = 0;
    }
    // if tc:IsAttack(atk) then return end
    if card::is_attack(d, tc, &[atk as u32]) {
        return;
    }
    // Make its ATK equal to the banished monster's original ATK
    // local e1=Effect.CreateEffect(e:GetHandler())
    let handler = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, handler);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    // e1:SetValue(atk)
    effect::set_value(d, e1, atk as i64);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // if tc:IsAttack(atk) and race&RACE_REPTILE==RACE_REPTILE then
    if card::is_attack(d, tc, &[atk as u32]) && race & RACE_REPTILE == RACE_REPTILE {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
