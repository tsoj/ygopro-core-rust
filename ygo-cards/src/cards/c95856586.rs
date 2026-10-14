//! Zexal Field
use super::prelude::*;

pub const ID: u32 = 95856586;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[35906693], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Your opponent cannot activate cards or effects in response to the activation of your cards and effects that target an Xyz Monster you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetOperation(s.chainop)
    effect::set_operation(d, e2, Some(operation(chainop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Attach 1 Xyz Monster from your Extra Deck or GY to a Xyz Monster(s) Special Summoned to your field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e3, EVENT_CUSTOM + ID);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.mtg)
    effect::set_target(d, e3, Some(cost_target(mtg)));
    // e3:SetOperation(s.mop)
    effect::set_operation(d, e3, Some(operation(mop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e3:SetLabelObject(g)
    effect::set_label_object(d, e3, Value::Group(CardSet::new()));
    // Keep track of Xyz Monster(s) Special Summoned to your field
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetLabelObject(e3)
    effect::set_label_object(d, e4, Value::Effect(e3));
    // e4:SetOperation(s.regop)
    effect::set_operation(d, e4, Some(operation(regop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // You can place 1 "Shining Draw" from your Deck on top of your Deck
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e5:SetCode(EVENT_PREDRAW)
    effect::set_code(d, e5, EVENT_PREDRAW);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetCondition(s.condition)
    effect::set_condition(d, e5, Some(condition(cond)));
    // e5:SetOperation(s.operation)
    effect::set_operation(d, e5, Some(operation(op)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.chfilter(c,tp)`
fn chfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::is_controler(d, c, tp) && card::is_location(d, c, LOCATION_MZONE)
}
/// `s.chainop(e,tp,eg,ep,ev,re,r,rp)`
fn chainop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    // if ep==tp and g and g:IsExists(s.chfilter,1,nil,tp) then
    if a.ep == a.tp {
        if let Some(g) = g {
            if group::is_exists(d, &g, &|d, c| chfilter(d, c, a.tp), 1, NO_EX) {
                // Duel.SetChainLimit(s.chainlm)
                duel::set_chain_limit(d, lua_fn(chainlm));
            }
        }
    }
}
/// `s.chainlm(e,rp,tp)`
fn chainlm(_d: &mut Duel, a: Args) -> bool {
    // return tp==rp
    a.p(2) == a.p(1)
}
/// `s.mcfilter(c,e,tp)`
fn mcfilter(d: &mut Duel, c: CardId, _e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:IsControler(tp)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::is_controler(d, c, tp)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local tg=eg:Filter(s.mcfilter,nil,e,tp)
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| mcfilter(d, c, a.e, a.tp), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do tc:RegisterFlagEffect(id,RESET_CHAIN,0,1) end
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e3 = effect::get_label_object(d, a.e).as_effect().nil();
        let mut g = effect::get_label_object(d, e3).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e3, Value::Group(g));
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,tp,0)
        let h = effect::get_handler(d, a.e).nil();
        duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(a.e), 0, a.tp, a.tp, 0);
    }
}
/// `s.mtg(e,tp,...,chk,chkc)`
fn mtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject():Filter(s.mcfilter,nil,e,tp)
    let lbl = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let g = group::filter(d, &lbl, &|d, c| mcfilter(d, c, a.e, a.tp), NO_EX);
    // if chkc then return g:IsContains(chkc) and s.mcfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && mcfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return #g>0 and Duel.GetFlagEffect(tp,id)==0
    //   and Duel.IsExistingMatchingCard(Card.IsType,tp,LOCATION_GRAVE|LOCATION_EXTRA,0,1,nil,TYPE_XYZ) end
    if a.chk == Some(0) {
        return !g.is_empty()
            && duel::get_flag_effect(d, a.tp, ID) == 0
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_type(d, c, TYPE_XYZ)), a.tp, LOCATION_GRAVE | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_CHAIN,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_CHAIN, 0, 1, 0);
    // if #g==1 then
    if g.len() == 1 {
        // Duel.SetTargetCard(g:GetFirst())
        let first = group::get_first(d, &g).nil();
        duel::set_target_card(d, first);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
        // local tc=g:Select(tp,1,1,nil)
        let tc = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, tc);
    }
    true
}
/// `s.mop(e,tp,...)`
fn mop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_XMATERIAL as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(Card.IsType),tp,LOCATION_GRAVE|LOCATION_EXTRA,0,1,1,tc,TYPE_XYZ)
        let g = duel::select_matching_card(
            d,
            a.tp,
            Some(&aux::necro_valley_filter(|d, c| card::is_type(d, c, TYPE_XYZ))),
            a.tp,
            LOCATION_GRAVE | LOCATION_EXTRA,
            0,
            1,
            1,
            false,
            tc.into(),
        )
        .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Overlay(tc,g,true)
            duel::overlay(d, tc, g, true);
        }
    }
}
/// `s.condition(e,tp,...)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_DECK,0,1,nil,35906693)
    duel::is_turn_player(d, a.tp) && duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[35906693])), a.tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.operation(e,tp,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstMatchingCard(Card.IsCode,tp,LOCATION_DECK,0,nil,35906693)
    let tc = duel::get_first_matching_card(d, Some(&|d, c| card::is_code(d, c, &[35906693])), a.tp, LOCATION_DECK, 0, NO_EX);
    // if tc then
    if let Some(tc) = tc {
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, a.tp);
        // Duel.MoveSequence(tc,0)
        duel::move_sequence(d, tc, 0, None);
        // Duel.ConfirmDecktop(tp,1)
        duel::confirm_decktop(d, a.tp, 1);
    }
}
