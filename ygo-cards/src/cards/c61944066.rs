//! Nouvelles Recipe Book "Recettes de Nouvellez"
use super::prelude::*;

pub const ID: u32 = 61944066;

// `s.listed_seris` (sic) is a misspelled field the engine never reads: left out.

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e0, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // While you control a "Nouvelles" Monster Card, all face-up monsters your opponent controls are changed to Attack Position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SET_POSITION)
    effect::set_code(d, e1, EFFECT_SET_POSITION);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetCondition(s.atkposcon)
    effect::set_condition(d, e1, Some(condition(atkposcon)));
    // e1:SetValue(POS_FACEUP_ATTACK)
    effect::set_value(d, e1, POS_FACEUP_ATTACK as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Each time a monster(s) is Tributed by a Ritual Monster's effect, your opponent pays 850 LP
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_RELEASE)
    effect::set_code(d, e2, EVENT_RELEASE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.lpcon)
    effect::set_condition(d, e2, Some(condition(lpcon)));
    // e2:SetOperation(s.lpop)
    effect::set_operation(d, e2, Some(operation(lpop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 "Recipe" or "Nouvelles" card from your Deck to your hand, except a Continuous Spell,
    // and if you do, increase the target's Level by 1
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3a, aux::stringid(ID, 1));
    // e3a:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_LVCHANGE)
    effect::set_category(d, e3a, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_LVCHANGE);
    // e3a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3a:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3a, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3a:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e3a, EVENT_CUSTOM + ID);
    // e3a:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3a, LOCATION_SZONE);
    // e3a:SetCountLimit(1)
    effect::set_count_limit(d, e3a, 1, 0, 0, 0);
    // e3a:SetCondition(function() return not Duel.IsPhase(PHASE_DAMAGE) end)
    effect::set_condition(d, e3a, Some(condition(phase_cond)));
    // e3a:SetTarget(s.thtg)
    effect::set_target(d, e3a, Some(cost_target(thtg)));
    // e3a:SetOperation(s.thop)
    effect::set_operation(d, e3a, Some(operation(thop)));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e3a:SetLabelObject(g)
    effect::set_label_object(d, e3a, Value::Group(CardSet::new()));
    // Register your Special Summoned Ritual Monsters
    // local e3b=Effect.CreateEffect(c)
    let e3b = effect::create_effect(d, c);
    // e3b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3b:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3b, EVENT_SPSUMMON_SUCCESS);
    // e3b:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3b, LOCATION_SZONE);
    // e3b:SetLabelObject(e3a)
    effect::set_label_object(d, e3b, Value::Effect(e3a));
    // e3b:SetOperation(s.regsumop)
    effect::set_operation(d, e3b, Some(operation(regsumop)));
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
}
/// `s.nouvfilter(c)`
fn nouvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NOUVELLES) and c:IsMonsterCard() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_NOUVELLES as u16]) && card::is_monster_card(d, c) && card::is_faceup(d, c)
}
/// `s.atkposcon(e)`
fn atkposcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.nouvfilter,e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&nouvfilter), p, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.lpcon(e,tp,eg,ep,ev,re,r,rp)`
fn lpcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re and re:IsMonsterEffect() and re:GetHandler():IsRitualMonster() and r&REASON_EFFECT>0 and eg:IsExists(Card.IsMonster,1,nil)
    let Some(re) = a.re else {
        return false;
    };
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    if !card::is_ritual_monster(d, h) {
        return false;
    }
    if a.r & REASON_EFFECT == 0 {
        return false;
    }
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_monster(d, c), 1, NO_EX)
}
/// `s.lpop(e,tp,...)`
fn lpop(d: &mut Duel, a: &EffArgs) {
    // if Duel.CheckLPCost(1-tp,850) then
    if duel::check_lp_cost(d, 1 - a.tp, 850) {
        // Duel.Hint(HINT_CARD,1-tp,id)
        duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
        // Duel.PayLPCost(1-tp,850)
        duel::pay_lp_cost(d, 1 - a.tp, 850);
    }
}
/// `function() return not Duel.IsPhase(PHASE_DAMAGE) end`
fn phase_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    !duel::is_phase(d, PHASE_DAMAGE)
}
/// `s.lvfilter(c,e,tp)`
fn lvfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRitualMonster() and c:IsSummonPlayer(tp) and c:IsFaceup() and c:IsCanBeEffectTarget(e)
    //   and c:IsLocation(LOCATION_MZONE) and c:HasLevel()
    card::is_ritual_monster(d, c)
        && card::is_summon_player(d, c, tp)
        && card::is_faceup(d, c)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_location(d, c, LOCATION_MZONE)
        && card::has_level(d, c)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_RECIPE,SET_NOUVELLES}) and not c:IsContinuousSpell() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_RECIPE as u16, SET_NOUVELLES as u16]) && !card::is_continuous_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,...,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject():Filter(s.lvfilter,nil,e,tp)
    let label = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let g = group::filter(d, &label, &|d, c| lvfilter(d, c, a.e, a.tp), NO_EX);
    // if chkc then return g:IsContains(chkc) and s.lvfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && lvfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return #g>0 and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return !g.is_empty() && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // local tc=nil
    // if #g>1 then
    let tc = if g.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
        // tc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sel)
    } else {
        // tc=g:GetFirst()
        group::get_first(d, &g)
    };
    // Duel.SetTargetCard(tc)
    let tc = tc.nil();
    duel::set_target_card(d, tc);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_LVCHANGE,tc,1,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, Some(CardSet::from_card(tc)), 1, a.tp, 1);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc and Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_HAND) then
    if let Some(sc) = sc {
        if duel::send_to_hand(d, sc, None, REASON_EFFECT, None) > 0 && card::is_location(d, sc, LOCATION_HAND) {
            // Duel.ConfirmCards(1-tp,sc)
            duel::confirm_cards(d, 1 - a.tp, sc);
            // local tc=Duel.GetFirstTarget()
            let tc = duel::first_target(d).nil();
            // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
            if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
                // Increase that monster's Level by 1
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e1:SetCode(EFFECT_UPDATE_LEVEL)
                effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
                // e1:SetValue(1)
                effect::set_value(d, e1, 1);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc, e1, false);
            }
        }
    }
}
/// `s.regsumop(e,tp,eg,ep,ev,re,r,rp)`
fn regsumop(d: &mut Duel, a: &EffArgs) {
    // local tg=eg:Filter(s.lvfilter,nil,e,tp)
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| lvfilter(d, c, a.e, a.tp), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do tc:RegisterFlagEffect(id,RESET_CHAIN,0,1) end
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e3a = effect::get_label_object(d, a.e).as_effect().nil();
        let mut g = effect::get_label_object(d, e3a).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g.clear();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e3a, Value::Group(g));
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,tp,0)
        let h = effect::get_handler(d, a.e).nil();
        duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(a.e), 0, a.tp, a.tp, 0);
    }
}
