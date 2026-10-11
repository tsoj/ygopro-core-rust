//! New World - Amritara
use super::prelude::*;

pub const ID: u32 = 62314831;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_VISAS_STARFROST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.effcon)
    effect::set_condition(d, e2, Some(condition(effcon)));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_VISAS_STARFROST),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_VISAS_STARFROST]))),
        a.tp,
        LOCATION_ONFIELD,
        LOCATION_ONFIELD,
        1,
        NO_EX,
    )
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    //     and c:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and c:IsFaceup()
    card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && card::is_location(d, c, LOCATION_GRAVE | LOCATION_REMOVED)
        && card::is_faceup(d, c)
}
// function s.effcon(e,tp,eg,ep,ev,re,r,rp)
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp), 1, NO_EX)
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // local g=eg:Filter(s.cfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, x| cfilter(d, x, tp), NO_EX);
    // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and g:IsExists(Card.IsCanBeSpecialSummoned,1,nil,e,0,tp,false,false,POS_FACEUP_DEFENSE)
    let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && group::is_exists(
            d,
            &g,
            &|d, x| card::is_can_be_special_summoned(d, x, a.e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None),
            1,
            NO_EX,
        );
    // local b2=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_TUNER),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    //     and g:IsExists(Card.IsAttackAbove,1,nil,1)
    let b2 = duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_type(d, x, TYPE_TUNER))), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
        && group::is_exists(d, &g, &|d, x| card::is_attack_above(d, x, 1), 1, NO_EX);
    // local b3=g:IsExists(Card.IsAbleToDeck,1,nil) and Duel.IsPlayerCanDraw(tp,1)
    let b3 = group::is_exists(d, &g, &|d, x| card::is_able_to_deck(d, x), 1, NO_EX) && duel::is_player_can_draw(d, tp, 1);
    // local b4=c:IsAbleToDeck() and Duel.IsExistingMatchingCard(aux.AND(Card.IsFieldSpell,Card.IsAbleToHand),tp,LOCATION_GRAVE,0,1,nil)
    let b4 = card::is_able_to_deck(d, c.nil())
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_field_spell(d, x) && card::is_able_to_hand(d, x, None)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 or b3 or b4 end
    if a.chk == Some(0) {
        return b1 || b2 || b3 || b4;
    }
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)},
    //     {b3,aux.Stringid(id,3)},
    //     {b4,aux.Stringid(id,4)})
    let op = duel::select_effect(
        d,
        tp,
        &[
            (b1, aux::stringid(ID, 1)),
            (b2, aux::stringid(ID, 2)),
            (b3, aux::stringid(ID, 3)),
            (b4, aux::stringid(ID, 4)),
        ],
    )
    .unwrap_or(0) as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    if op == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_ATKCHANGE)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE);
    } else if op == 3 {
        // e:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_TODECK + CATEGORY_DRAW);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), 1, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    } else if op == 4 {
        // e:SetCategory(CATEGORY_TODECK+CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TODECK + CATEGORY_TOHAND);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,c,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(c.nil())), 1, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_GRAVE as i32);
    }
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if #g==0 and op~=4 then return end
    if g.is_empty() && op != 4 {
        return;
    }
    // Special Summon 1 of them in Defense Position
    // if op==1 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if op == 1 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:FilterSelect(tp,Card.IsCanBeSpecialSummoned,1,1,nil,e,0,tp,false,false,POS_FACEUP_DEFENSE)
        let sg = group::filter_select(
            d,
            &g,
            tp,
            &|d, x| card::is_can_be_special_summoned(d, x, a.e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None),
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    // 1 Tuner on the field gains ATK
    // elseif op==2 and g:IsExists(Card.IsAttackAbove,1,nil,1) then
    } else if op == 2 && group::is_exists(d, &g, &|d, x| card::is_attack_above(d, x, 1), 1, NO_EX) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
        // local tuner=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsType,TYPE_TUNER),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil):GetFirst()
        let tuner_g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::faceup_filter(|d, x| card::is_type(d, x, TYPE_TUNER))),
            tp,
            LOCATION_MZONE,
            LOCATION_MZONE,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        let tuner = group::get_first(d, &tuner_g);
        // if tuner then
        if let Some(tuner) = tuner {
            // Duel.HintSelection(tuner,true)
            duel::hint_selection(d, tuner, true);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
            // local tc=g:FilterSelect(tp,Card.IsAttackAbove,1,1,nil,1):GetFirst()
            let tc_g = group::filter_select(d, &g, tp, &|d, x| card::is_attack_above(d, x, 1), 1, 1, false, NO_EX).nil();
            let tc = group::get_first(d, &tc_g).nil();
            // Duel.HintSelection(tc,true)
            duel::hint_selection(d, tc, true);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c.nil());
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(tc:GetAttack()/2)
            let atk = card::get_attack(d, tc);
            effect::set_value(d, e1, lua_int(atk as f64 / 2.0));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tuner:RegisterEffect(e1)
            card::register_effect(d, tuner, e1, false);
        }
    // Shuffle 1 of them into the Deck and draw 1 card
    // elseif op==3 then
    } else if op == 3 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // local tc=g:FilterSelect(tp,Card.IsAbleToDeck,1,1,nil):GetFirst()
        let tc_g = group::filter_select(d, &g, tp, &|d, x| card::is_able_to_deck(d, x), 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &tc_g);
        // if tc then
        if let Some(tc) = tc {
            // Duel.HintSelection(tc,true)
            duel::hint_selection(d, tc, true);
            // if Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_DECK|LOCATION_EXTRA) then
            if duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
                && card::is_location(d, tc, LOCATION_DECK | LOCATION_EXTRA)
            {
                // if tc:IsControler(tp) and tc:IsLocation(LOCATION_DECK) then Duel.ShuffleDeck(tp) end
                if card::is_controler(d, tc, tp) && card::is_location(d, tc, LOCATION_DECK) {
                    duel::shuffle_deck(d, tp);
                }
                // Duel.Draw(tp,1,REASON_EFFECT)
                duel::draw(d, tp, 1, REASON_EFFECT);
            }
        }
    // Shuffle this card into the Deck and add 1 Field Spell from your GY to your hand
    // elseif op==4 and Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and c:IsLocation(LOCATION_DECK) then
    } else if op == 4
        && duel::send_to_deck(d, c.nil(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, c.nil(), LOCATION_DECK)
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local th=Duel.SelectMatchingCard(tp,aux.AND(Card.IsFieldSpell,Card.IsAbleToHand),tp,LOCATION_GRAVE,0,1,1,nil)
        let th = duel::select_matching_card(d, tp, Some(&|d, x| card::is_field_spell(d, x) && card::is_able_to_hand(d, x, None)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #th>0 then
        if !th.is_empty() {
            // Duel.SendtoHand(th,nil,REASON_EFFECT)
            duel::send_to_hand(d, th.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,th)
            duel::confirm_cards(d, 1 - tp, th);
        }
    }
}
