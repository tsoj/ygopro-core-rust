//! Super Critical
use super::prelude::*;

pub const ID: u32 = 100459015;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_EVENT_PLAYER)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY + EFFECT_FLAG_EVENT_PLAYER, 0);
    // e2a:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e2a, EVENT_CUSTOM + ID);
    // e2a:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2a, LOCATION_SZONE);
    // e2a:SetTarget(s.efftg)
    effect::set_target(d, e2a, Some(cost_target(efftg)));
    // e2a:SetOperation(s.effop)
    effect::set_operation(d, e2a, Some(operation(effop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=Effect.CreateEffect(c)
    let e2b = effect::create_effect(d, c);
    // e2b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2b:SetProperty(EFFECT_CANNOT_DISABLE)
    effect::set_property(d, e2b, EFFECT_CANNOT_DISABLE, 0);
    // e2b:SetCode(EVENT_TOSS_DICE)
    effect::set_code(d, e2b, EVENT_TOSS_DICE);
    // e2b:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2b, LOCATION_SZONE);
    // e2b:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e2b, Some(operation(e2b_op)));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` of e2b: raises the custom event for each die showing 1
fn e2b_op(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsDamageStep() then return end
    if duel::is_damage_step(d, None) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local dice_results={Duel.GetDiceResult()}
    let dice_results = duel::get_dice_result(d);
    // local self_dice_count=aux.GetDiceCountSelfFromEv(ev)
    let self_dice_count = aux::get_dice_count_self_from_ev(a.ev) as usize;
    // for i=1,#dice_results do
    for (i, &result) in dice_results.iter().enumerate() {
        // if dice_results[i]==1 then
        if result == 1 {
            // local event_player=i<=self_dice_count and ep or 1-ep
            let event_player = if i + 1 <= self_dice_count { a.ep } else { 1 - a.ep };
            // Duel.RaiseSingleEvent(c,EVENT_CUSTOM+id,re,r,rp,event_player,ev)
            duel::raise_single_event(d, c, EVENT_CUSTOM + ID, a.re, a.r, a.rp, event_player, a.ev);
        }
    }
    // return false
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c.roll_dice and c:IsAbleToHand()
    crate::card_info(d, c).roll_dice && card::is_able_to_hand(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, sg);
    }
}
/// `s.desfilter(c,tp)`
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,1,c)
    duel::is_existing_matching_card(d, Some(&card::is_negatable_monster), tp, 0, LOCATION_MZONE, 1, c.into())
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let g = duel::get_field_group(d, tp, 0, LOCATION_MZONE);
    // local option_1=#g>0
    let option_1 = !g.is_empty();
    // local option_2=Duel.IsExistingMatchingCard(s.desfilter,tp,0,LOCATION_MZONE,1,nil,tp)
    let option_2 = duel::is_existing_matching_card(d, Some(&|d, x| desfilter(d, x, tp)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local option_3=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(Card.IsCanBeSpecialSummoned,tp,LOCATION_GRAVE,0,1,nil,e,0,tp,false,false)
    let option_3 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(
            d,
            Some(&|d, x| card::is_can_be_special_summoned(d, x, a.e, 0, tp, false, false, None, None, None)),
            tp,
            LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        );
    // if chk==0 then return (option_1 or option_2 or option_3) and not c:HasFlagEffect(id+100+tp) end
    if a.chk == Some(0) {
        return (option_1 || option_2 || option_3) && !card::has_flag_effect(d, c, ID + 100 + tp as u32, None);
    }
    // c:RegisterFlagEffect(id+100+tp,RESET_EVENT|RESETS_STANDARD|RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID + 100 + tp as u32, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 0, 1, 0, 0);
    // local choice=Duel.SelectEffect(tp,
    //     {option_1,aux.Stringid(id,3)},
    //     {option_2,aux.Stringid(id,4)},
    //     {option_3,aux.Stringid(id,5)})
    let choice = duel::select_effect(
        d,
        tp,
        &[(option_1, aux::stringid(ID, 3)), (option_2, aux::stringid(ID, 4)), (option_3, aux::stringid(ID, 5))],
    );
    // e:GetChainData().choice=choice
    let choice_value = match choice {
        Some(x) => Value::Int(x as i64),
        None => Value::Nil,
    };
    chain::set_effect_data(d, a.e, None, chain::field::CARD, choice_value);
    match choice {
        // if choice==1 then
        Some(1) => {
            // e:SetCategory(CATEGORY_DESTROY)
            effect::set_category(d, a.e, CATEGORY_DESTROY);
            // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
            duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
        }
        // elseif choice==2 then
        Some(2) => {
            // e:SetCategory(CATEGORY_DESTROY+CATEGORY_DISABLE)
            effect::set_category(d, a.e, CATEGORY_DESTROY + CATEGORY_DISABLE);
            // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
            duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
            // Duel.SetOperationInfo(0,CATEGORY_DISABLE,nil,1,1-tp,LOCATION_MZONE)
            duel::set_operation_info(d, 0, CATEGORY_DISABLE, None, 1, 1 - tp, LOCATION_MZONE as i32);
        }
        // elseif choice==3 then
        Some(3) => {
            // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
            effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
            // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
            duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
        }
        _ => {}
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local choice=e:GetChainData().choice
    let choice = match chain::effect_data(d, a.e, None, chain::field::CARD) {
        Value::Int(x) => x,
        _ => 0,
    };
    match choice {
        // if choice==1 then
        1 => {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            // local g=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_MZONE,1,3,nil)
            let g = duel::select_matching_card(d, tp, None, tp, 0, LOCATION_MZONE, 1, 3, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.HintSelection(g)
                duel::hint_selection(d, g.clone(), true);
                // Duel.Destroy(g,REASON_EFFECT)
                duel::destroy(d, g, REASON_EFFECT, None, None);
            }
        }
        // elseif choice==2 then
        2 => {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            // local dg=Duel.SelectMatchingCard(tp,s.desfilter,tp,0,LOCATION_MZONE,1,1,nil,tp)
            let mut dg = duel::select_matching_card(d, tp, Some(&|d, x| desfilter(d, x, tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
            // if #dg==0 then
            if dg.is_empty() {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
                // dg=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_MZONE,1,1,nil)
                dg = duel::select_matching_card(d, tp, None, tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
                // if #dg==0 then return end
                if dg.is_empty() {
                    return;
                }
            }
            // Duel.HintSelection(dg)
            duel::hint_selection(d, dg.clone(), true);
            // if Duel.Destroy(dg,REASON_EFFECT)>0 then
            if duel::destroy(d, dg, REASON_EFFECT, None, None) > 0 {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
                // local sc=Duel.SelectMatchingCard(tp,Card.IsNegatableMonster,tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
                let sel = duel::select_matching_card(d, tp, Some(&card::is_negatable_monster), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
                // if sc then
                if let Some(sc) = group::get_first(d, &sel) {
                    // Duel.HintSelection(sc)
                    duel::hint_selection(d, CardSet::from_card(sc), true);
                    // sc:NegateEffects(e:GetHandler())
                    let h = effect::get_handler(d, a.e).nil();
                    card::negate_effects(d, sc, h, None, false, None);
                }
            }
        }
        // elseif choice==3 then
        3 => {
            // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
            if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
                return;
            }
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local g=Duel.SelectMatchingCard(tp,Card.IsCanBeSpecialSummoned,tp,LOCATION_GRAVE,0,1,1,nil,e,0,tp,false,false)
            let g = duel::select_matching_card(
                d,
                tp,
                Some(&|d, x| card::is_can_be_special_summoned(d, x, a.e, 0, tp, false, false, None, None, None)),
                tp,
                LOCATION_GRAVE,
                0,
                1,
                1,
                false,
                NO_EX,
            )
            .nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
        _ => {}
    }
}
