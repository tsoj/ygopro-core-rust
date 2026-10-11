//! Seventh Ascension
//! Scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 23153227;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_SEVENTH, SET_BARIANS, SET_RANK_UP_MAGIC],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
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
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Copy effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.copycon)
    effect::set_condition(d, e2, Some(condition(copycon)));
    // e2:SetCost(s.copycost)
    effect::set_cost(d, e2, Some(cost_target(copycost)));
    // e2:SetTarget(s.copytg)
    effect::set_target(d, e2, Some(cost_target(copytg)));
    // e2:SetOperation(s.copyop)
    effect::set_operation(d, e2, Some(operation(copyop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,dct)`
fn filter(d: &mut Duel, c: CardId, dct: i32) -> bool {
    // return ((((c:IsSetCard(SET_SEVENTH) and not c:IsCode(id)) or c:IsSetCard(SET_BARIANS)) and c:IsSpellTrap())
    //     or (c:IsSetCard(SET_RANK_UP_MAGIC) and c:IsQuickPlaySpell()))
    //     and (c:IsAbleToHand() or dct>1)
    let seventh = card::is_set_card(d, c, &[SET_SEVENTH as u16]) && !card::is_code(d, c, &[ID]);
    let part_a = (seventh || card::is_set_card(d, c, &[SET_BARIANS as u16])) && card::is_spell_trap(d, c);
    let part_b = card::is_set_card(d, c, &[SET_RANK_UP_MAGIC as u16]) && card::is_quick_play_spell(d, c);
    (part_a || part_b) && (card::is_able_to_hand(d, c, None) || dct > 1)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local dct=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
    let dct = duel::get_field_group_count(d, tp, LOCATION_DECK, 0);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,dct) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, dct)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local dct=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
    let dct = duel::get_field_group_count(d, tp, LOCATION_DECK, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local tc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,dct):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, dct)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // aux.ToHandOrElse(tc,tp,function(c) return dct>1 end,
        //     function(c)
        //         Duel.ShuffleDeck(tp)
        //         Duel.MoveSequence(c,SEQ_DECKTOP)
        //         Duel.ConfirmDecktop(tp,1) end,
        //     aux.Stringid(id,3))
        aux::to_hand_or_else(
            d,
            tc,
            tp,
            Some(&|_d, _c| dct > 1),
            Some(&|d, _g| {
                duel::shuffle_deck(d, tp);
                duel::move_sequence(d, tc, SEQ_DECKTOP as u8, None);
                duel::confirm_decktop(d, tp, 1);
                0
            }),
            Some(aux::stringid(ID, 3)),
        );
    }
}
/// `s.copycon(e,tp,eg,ep,ev,re,r,rp)`
fn copycon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsSummonLocation,tp,0,LOCATION_MZONE,1,nil,LOCATION_EXTRA)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_summon_location(d, c, LOCATION_EXTRA)), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.copycost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn copycost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsAbleToRemoveAsCost() end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, h, None);
    }
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_COST)
    duel::remove(d, h, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.copyfilter(c)`
fn copyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToGraveAsCost() and c:IsSetCard(SET_RANK_UP_MAGIC) and c:IsSpell()
    //     and c:CheckActivateEffect(true,true,false)~=nil
    card::is_able_to_grave_as_cost(d, c)
        && card::is_set_card(d, c, &[SET_RANK_UP_MAGIC as u16])
        && card::is_spell(d, c)
        && card::check_activate_effect(d, c, true, true, false).is_some()
}
/// `s.copytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn copytg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then
    if a.chkc().is_some() {
        // local te=e:GetLabelObject()
        let _te = effect::get_label_object(d, a.e);
        // return tg and tg(e,tp,eg,ep,ev,re,r,rp,0,chkc)
        // (`tg` is a global here, which is nil: the result is nil, i.e. false)
        return false;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, a.e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.IsExistingMatchingCard(s.copyfilter,tp,LOCATION_HAND,0,1,nil)
        return duel::is_existing_matching_card(d, Some(&copyfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.copyfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&copyfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if not Duel.SendtoGrave(g,REASON_COST) then return end
    // (a Lua number is always true, so the SendtoGrave count never returns here)
    duel::send_to_grave(d, g.clone(), REASON_COST, None, None);
    // local te=g:GetFirst():CheckActivateEffect(true,true,false)
    let gc = group::get_first(d, &g).nil();
    let te = card::check_activate_effect(d, gc, true, true, false).map(|x| x.0).nil();
    // e:SetLabel(te:GetLabel())
    let te_label = effect::get_label(d, te);
    effect::set_label(d, a.e, &[te_label]);
    // e:SetLabelObject(te:GetLabelObject())
    let te_lo = effect::get_label_object(d, te);
    effect::set_label_object(d, a.e, te_lo);
    // local tg=te:GetTarget()
    // if tg then
    if let Some(tg) = effect::get_target(d, te) {
        // tg(e,tp,eg,ep,ev,re,r,rp,1)
        tg.call(d, &copy_args(a, Some(Value::Int(1))));
    }
    // te:SetLabel(e:GetLabel())
    let e_label = effect::get_label(d, a.e);
    effect::set_label(d, te, &[e_label]);
    // te:SetLabelObject(e:GetLabelObject())
    let e_lo = effect::get_label_object(d, a.e);
    effect::set_label_object(d, te, e_lo);
    // e:SetLabelObject(te)
    effect::set_label_object(d, a.e, Value::Effect(te));
    // Duel.ClearOperationInfo(0)
    duel::clear_operation_info(d, 0);
    true
}
/// The arguments `(e,tp,eg,ep,ev,re,r,rp)` of the copied effect, with an optional 9th `chk`
fn copy_args(a: &EffArgs, chk: Option<Value>) -> Vec<Value> {
    let mut v = vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        a.re.map_or(Value::Nil, Value::Effect),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    if let Some(chk) = chk {
        v.push(chk);
    }
    v
}
/// `s.copyop(e,tp,eg,ep,ev,re,r,rp)`
fn copyop(d: &mut Duel, a: &EffArgs) {
    // local te=e:GetLabelObject()
    // if te then
    if let Value::Effect(te) = effect::get_label_object(d, a.e) {
        // e:SetLabel(te:GetLabel())
        let te_label = effect::get_label(d, te);
        effect::set_label(d, a.e, &[te_label]);
        // e:SetLabelObject(te:GetLabelObject())
        let te_lo = effect::get_label_object(d, te);
        effect::set_label_object(d, a.e, te_lo);
        // local op=te:GetOperation()
        // if op then op(e,tp,eg,ep,ev,re,r,rp) end
        if let Some(op) = effect::get_operation(d, te) {
            op.call(d, &copy_args(a, None));
        }
        // te:SetLabel(e:GetLabel())
        let e_label = effect::get_label(d, a.e);
        effect::set_label(d, te, &[e_label]);
        // te:SetLabelObject(e:GetLabelObject())
        let e_lo = effect::get_label_object(d, a.e);
        effect::set_label_object(d, te, e_lo);
    }
}
